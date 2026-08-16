//! GGA_C_GAPC lxc pol — lxc_pol part 27 (v4rho2sigma2_6) CSE chunk 1109/1310 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gapc_lxc_pol_part27_v4rho2sigma2_6_chunk1109(t11745: f64, t18331: f64, t11387: f64, t7204: f64, t7557: f64, t11483: f64, t11749: f64, t2787: f64, t33701: f64, t33704: f64, t33707: f64, t33710: f64, t33714: f64, t33717: f64, t33719: f64, t33726: f64) -> f64 {
    let t33728 = t18331 * t11745;
    let t33731 = t7204 * t11387 * t7557;
    let t33734 = t2787 * t11483 * t11749;
    let t33736 = -0.21720231316129303386e-4_f64 * t33701 - 0.21720231316129303386e-4_f64 * t33704 - 0.10860115658064651693e-4_f64 * t33707 - 0.20611878024038059902e-5_f64 * t33710 + 0.36647919126739670507e-5_f64 * t33714 - 0.36872409820556640627e-8_f64 * t33717 + 0.63252766927083333336e-6_f64 * t33719 + 0.20240885416666666668e-4_f64 * t33726 - 0.5686343261418565457e-6_f64 * t33728 - 0.5686343261418565457e-6_f64 * t33731 + 0.2318836277704281739e-4_f64 * t33734;
    t33736
}
