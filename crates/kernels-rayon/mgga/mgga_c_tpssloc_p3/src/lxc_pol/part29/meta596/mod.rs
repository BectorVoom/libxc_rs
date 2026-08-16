//! MGGA_C_TPSSLOC lxc pol kernel — _part29_v4rho3sigma_5 meta596 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2026;
use chunk1::mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2027;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_meta596(t1905: f64, t81686: f64, t9537: f64, t23004: f64, t23110: f64, t23185: f64, t23005: f64, t6579: f64, t23181: f64, t2587: f64, t81151: f64, t23172: f64, t23150: f64, t814: f64, t133: f64, t1891: f64, t6601: f64, t80953: f64, t22816: f64, t23104: f64, t80967: f64, t6612: f64, t812: f64, t836: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t81689, t81691, t81697, t81704, t81715, t81716) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2026(t1905, t81686, t9537, t23004, t23110, t23185, t23005, t6579, t23181, t2587, t81151, t23172);
        let (t81717, t81718, t81736, t81743, t81749) = mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2027(t81716, t23150, t814, t133, t1891, t6601, t80953, t22816, t23104, t80967, t6612, t812, t836);
    (t81689, t81691, t81697, t81704, t81715, t81717, t81718, t81736, t81743, t81749)
}
