//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 795/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk795(t7930: f64, t2203: f64, t3046: f64, t2215: f64, t218: f64, t3061: f64, t675: f64, t3065: f64, t1174: f64, t6149: f64, t6165: f64, t1171: f64, t2196: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7957 = 2.0_f64 / 3.0_f64 * t7930;
    let t7966 = t2203 * t3046;
    let t7972 = t2215 * t3046;
    let t7979 = t218 * t675 * t3061;
    let t7980 = 0.32862666666666666666e0_f64 * t7979;
    let t7982 = t218 * t675 * t3065;
    let t7983 = 0.32862666666666666666e0_f64 * t7982;
    let t7996 = t6149 * t1174;
    let t7999 = t6165 * t1174;
    let t8009 = t1171 * t2196;
    (t7957, t7966, t7972, t7979, t7980, t7982, t7983, t7996, t7999, t8009)
}
