//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1208/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1208(t2860: f64, t5737: f64, t10813: f64, t5758: f64, t2841: f64, t5689: f64, t2403: f64, t5720: f64, t5723: f64, t5717: f64, t2929: f64, t5769: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t59920 = t5737 * t2860;
    let t59941 = t5758 * t10813;
    let t59959 = t5689 * t2841;
    let t60168 = t2403 * t5720;
    let t60173 = t2403 * t5723;
    let t60204 = t2403 * t5717;
    let t60343 = t5769 * t2929;
    (t59920, t59941, t59959, t60168, t60173, t60204, t60343)
}
