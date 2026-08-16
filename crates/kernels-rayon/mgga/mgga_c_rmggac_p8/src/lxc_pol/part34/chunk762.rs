//! MGGA_C_RMGGAC lxc pol — lxc_pol part 34 (v4rho2sigma2_7) CSE chunk 762/1097 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part34_v4rho2sigma2_7_chunk762(t236: f64, t31817: f64, t14125: f64, t68448: f64, t15339: f64, t3119: f64, t34855: f64, t14053: f64, t7715: f64, t14056: f64, t14059: f64, t14012: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t73785 = t236 * t31817;
    let t73787 = t68448 * t14125 * t73785;
    let t73790 = t15339 * t34855 * t3119;
    let t73791 = t73790 * t14053;
    let t73793 = t15339 * t7715;
    let t73794 = t73793 * t3119;
    let t73795 = t73794 * t14056;
    let t73797 = t73794 * t14059;
    let t73799 = t73794 * t14012;
    (t73785, t73787, t73791, t73793, t73794, t73795, t73797, t73799)
}
