//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 519/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk519(t1219: f64, t2110: f64, t1458: f64, t2240: f64, t1173: f64, t476: f64, t458: f64, t2250: f64, t4265: f64, t139: f64, t201: f64, t41: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t6221 = t2110 * t1219;
    let t6241 = t2240 * t1458;
    let t6256 = t476 * t1173;
    let t6267 = t476 * t458;
    let t6275 = t4265 * t2250;
    let t6278 = t139 * t201 * t41;
    (t6221, t6241, t6256, t6267, t6275, t6278)
}
