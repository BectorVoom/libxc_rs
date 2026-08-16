//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 232/1086 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk232(t1001: f64, t116: f64, t982: f64, t979: f64, t136: f64, t852: f64, t856: f64, t934: f64, t939: f64, t977: f64, t218: f64, t217: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t1002 = t116 * t1001;
    let t1003 = t982 * t1002;
    let t1004 = t979 * t1003;
    let t1006 = t852 * t136 - 0.193e0_f64 * t856 * t934 - 0.13265555555555555555e-1_f64 * t939 + 0.99491666666666666664e-2_f64 * t977 - 0.99491666666666666664e-2_f64 * t1004;
    let t1007 = t1006 * t218;
    let t1008 = t217 * t217;
    (t1002, t1003, t1004, t1006, t1007, t1008)
}
