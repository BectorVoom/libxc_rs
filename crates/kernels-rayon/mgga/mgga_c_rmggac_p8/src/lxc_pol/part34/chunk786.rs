//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 786/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk786(t14236: f64, t14243: f64, t2067: f64, t55986: f64, t15371: f64, t69568: f64, t68524: f64, t14063: f64, t3151: f64, t8450: f64, t15363: f64, t3140: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t74159 = t14236 * t14243 * t2067 * t55986;
    let t74161 = t69568 * t15371;
    let t74163 = t68524 * t15371;
    let t74166 = t8450 * t14063 * t3151;
    let t74168 = t68524 * t15363;
    let t74170 = t8450 * t3140;
    (t74159, t74161, t74163, t74166, t74168, t74170)
}
