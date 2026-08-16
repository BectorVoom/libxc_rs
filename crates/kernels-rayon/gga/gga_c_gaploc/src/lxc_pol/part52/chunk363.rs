//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 363/1013 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk363(t3009: f64, t723: f64, t1445: f64, t1036: f64, t1966: f64, t1991: f64, t2028: f64, t2033: f64, t2087: f64, t2601: f64, t2605: f64, t2608: f64, t2613: f64, t2619: f64, t2629: f64, t2658: f64, t2681: f64, t2687: f64, t2976: f64, t2979: f64, t2989: f64, t2992: f64, t2995: f64, t3002: f64, t3006: f64, t784: f64, t797: f64, t813: f64, t833: f64) -> f64 {
    let t3010 = t3009 * t723;
    let t3011 = t1445 * t3010;
    let t3014 = -0.39722766613167140743e-1_f64 * t2976 * t2028 + 0.39722766613167140743e-1_f64 * t2033 * t2979 + 0.29792074959875355558e-1_f64 * t2601 - 0.29792074959875355558e-1_f64 * t2605 + 0.25561950635947166451e0_f64 * t2608 - 0.29792074959875355558e-1_f64 * t2613 + 0.19171462976960374838e0_f64 * t2619 - 0.59584149919750711116e-1_f64 * t2629 + 0.29792074959875355558e-1_f64 * t2658 + 0.30674340763136599741e1_f64 * t833 * t2989 - 0.23833659967900284446e0_f64 * t797 * t2992 - 0.30674340763136599741e1_f64 * t813 * t2995 + 0.23833659967900284446e0_f64 * t1036 * t784 - 0.38342925953920749676e0_f64 * t2681 + 0.38342925953920749676e0_f64 * t2687 + 0.51123901271894332902e0_f64 * t1991 * t3002 - 0.51123901271894332902e0_f64 * t1966 * t3006 - 0.69017266717057349418e1_f64 * t2087 * t3011;
    t3014
}
