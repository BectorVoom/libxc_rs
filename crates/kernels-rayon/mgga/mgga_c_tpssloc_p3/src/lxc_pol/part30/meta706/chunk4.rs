//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2324/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2324(t89953: f64, t97999: f64, t10143: f64, t1649: f64, t25374: f64, t5966: f64, t776: f64, t4303: f64, t23788: f64, t67164: f64, t16944: f64, t25891: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t100682 = t89953 * t97999;
    let t100688 = t10143 * t1649;
    let t100689 = t100688 * t25374;
    let t100692 = t5966 * t776;
    let t100696 = t1649 * t4303;
    let t100705 = t23788 * t67164;
    let t100708 = t25891 * t16944;
    (t100682, t100689, t100692, t100696, t100705, t100708)
}
