//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1104/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1104(t117: f64, t4179: f64, t6559: f64, t229: f64, t268: f64, t131: f64, t2587: f64, t81142: f64, t1905: f64, t9537: f64, t81151: f64, t23172: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t81640 = t6559 * t4179 * t117;
    let t81651 = t6559 * t229 * t268;
    let t81686 = t81142 * t2587 * t131;
    let t81688 = t81686 * t9537 * t1905;
    let t81715 = t81151 * t2587;
    let t81716 = t81715 * t23172;
    (t81640, t81651, t81686, t81688, t81715, t81716)
}
