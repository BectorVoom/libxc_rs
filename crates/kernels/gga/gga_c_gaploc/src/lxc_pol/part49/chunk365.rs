//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 365/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk365<F: Float>(t3009: F, t723: F, t1445: F, t1036: F, t1966: F, t1991: F, t2028: F, t2033: F, t2087: F, t2601: F, t2605: F, t2608: F, t2613: F, t2619: F, t2629: F, t2658: F, t2681: F, t2687: F, t2976: F, t2979: F, t2989: F, t2992: F, t2995: F, t3002: F, t3006: F, t784: F, t797: F, t813: F, t833: F) -> F {
    let t3010 = t3009 * t723;
    let t3011 = t1445 * t3010;
    let t3014 = -F::cast_from(0.39722766613167140743e-1_f64) * t2976 * t2028 + F::cast_from(0.39722766613167140743e-1_f64) * t2033 * t2979 + F::cast_from(0.29792074959875355558e-1_f64) * t2601 - F::cast_from(0.29792074959875355558e-1_f64) * t2605 + F::cast_from(0.25561950635947166451e0_f64) * t2608 - F::cast_from(0.29792074959875355558e-1_f64) * t2613 + F::cast_from(0.19171462976960374838e0_f64) * t2619 - F::cast_from(0.59584149919750711116e-1_f64) * t2629 + F::cast_from(0.29792074959875355558e-1_f64) * t2658 + F::cast_from(0.30674340763136599741e1_f64) * t833 * t2989 - F::cast_from(0.23833659967900284446e0_f64) * t797 * t2992 - F::cast_from(0.30674340763136599741e1_f64) * t813 * t2995 + F::cast_from(0.23833659967900284446e0_f64) * t1036 * t784 - F::cast_from(0.38342925953920749676e0_f64) * t2681 + F::cast_from(0.38342925953920749676e0_f64) * t2687 + F::cast_from(0.51123901271894332902e0_f64) * t1991 * t3002 - F::cast_from(0.51123901271894332902e0_f64) * t1966 * t3006 - F::cast_from(0.69017266717057349418e1_f64) * t2087 * t3011;
    t3014
}
