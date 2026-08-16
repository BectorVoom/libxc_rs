//! MGGA_C_RMGGAC lxc pol — lxc_pol part 37 (v4rho2sigma2_10) CSE chunk 633/1128 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part37_v4rho2sigma2_10_chunk633(t3134: f64, t511: f64, t27: f64, t498: f64, t3142: f64, t676: f64, t880: f64, t2144: f64, t495: f64, t1968: f64, t7427: f64, t1966: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t16058 = t3134 * t511;
    let t16059 = t27 * t498;
    let t16064 = t3142 * t511;
    let t16069 = t676 * t880;
    let t16074 = t676 * t2144;
    let t16129 = t676 * t511;
    let t16130 = t27 * t495;
    let t16155 = t7427 * t1968;
    let t16156 = t1966 * t16155;
    (t16058, t16059, t16064, t16069, t16074, t16129, t16130, t16155, t16156)
}
