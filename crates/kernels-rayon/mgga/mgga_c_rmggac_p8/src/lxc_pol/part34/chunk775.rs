//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 775/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk775(t14116: f64, t14125: f64, t8421: f64, t15361: f64, t498: f64, t14236: f64, t2067: f64, t68471: f64, t321: f64, t69629: f64, t333: f64, t69588: f64) -> (f64, f64, f64, f64) {
    let t73984 = t14116 * t14125 * t8421;
    let t73986 = t15361 * t498;
    let t73989 = t14236 * t68471 * t2067 * t73986;
    let t73991 = t15361 * t321;
    let t73994 = t14236 * t69629 * t2067 * t73991;
    let t73996 = t15361 * t333;
    let t73999 = t14236 * t69588 * t2067 * t73996;
    (t73984, t73989, t73994, t73999)
}
