//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1910/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1910(t28248: f64, t81547: f64, t5660: f64, t606: f64, t17109: f64, t25: f64, t5664: f64, t5397: f64, t776: f64, t868: f64, t25373: f64, t23168: f64, t28288: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t98079 = t81547 * t28248;
    let t98082 = t606 * t5660;
    let t98086 = t25 * t17109;
    let t98091 = t606 * t5664;
    let t98094 = t5397 * t776;
    let t98102 = t5660 * t868;
    let t98103 = t25373 * t98102;
    let t98111 = t28248 * t868;
    let t98112 = t25373 * t98111;
    let t98117 = t23168 * t28288;
    (t98079, t98082, t98086, t98091, t98094, t98102, t98103, t98111, t98112, t98117)
}
