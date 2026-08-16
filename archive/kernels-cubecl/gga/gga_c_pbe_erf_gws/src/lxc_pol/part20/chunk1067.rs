//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 1067/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk1067<F: Float>(t11984: F, t2124: F, t2119: F, t3912: F, t6342: F, t3814: F, t8827: F, t9665: F, t2083: F, t3780: F, t3259: F, t3257: F) -> (F, F, F, F, F, F, F) {
    let t11986 = t11984 * t2124 / F::cast_from(48.0_f64);
    let t11987 = t3912 * t2119;
    let t11989 = t11987 * t6342 / F::cast_from(48.0_f64);
    let t11990 = t8827 * t3814;
    let t11991 = t9665 * t11990;
    let t11994 = t3780 * t2083;
    let t11995 = t11994 * t3259;
    let t11996 = t3257 * t11995;
    (t11986, t11989, t11990, t11991, t11994, t11995, t11996)
}
