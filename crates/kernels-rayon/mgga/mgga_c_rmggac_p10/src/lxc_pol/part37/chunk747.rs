//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 747/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk747(t321: f64, t5259: f64, t71949: f64, t352: f64, t5148: f64, t22: f64, t699: f64, t3814: f64, t3191: f64, t7561: f64, t2211: f64, t838: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t71951 = t5259 * t71949 * t321;
    let t71960 = t5148 * t71949 * t352;
    let t71982 = t699 * t22;
    let t71983 = t3814 * t71982;
    let t72010 = t3191 * t7561;
    let t72011 = 0.33335697577410973224e-1_f64 * t72010;
    let t72019 = t2211 * t22;
    let t72020 = t838 * t72019;
    (t71951, t71960, t71982, t71983, t72011, t72019, t72020)
}
