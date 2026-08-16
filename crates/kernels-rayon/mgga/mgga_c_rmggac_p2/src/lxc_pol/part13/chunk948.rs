//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 948/1127 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk948(t25877: f64, t40833: f64, t40687: f64, t793: f64, t38746: f64, t7785: f64, t39689: f64, t39671: f64, t7829: f64, t39685: f64, t39675: f64, t7782: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t40834 = t25877 * t40833;
    let t40842 = t793 * t40687;
    let t40844 = t7785 * t38746;
    let t40846 = t7785 * t39689;
    let t40850 = t7829 * t39671;
    let t40852 = t7829 * t39685;
    let t40854 = t7782 * t39675;
    (t40834, t40842, t40844, t40846, t40850, t40852, t40854)
}
