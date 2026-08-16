//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 893/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk893(t225: f64, t24200: f64, t24237: f64, t24235: f64, t111: f64, t7415: f64, t112: f64, t24954: f64, t24542: f64, t25: f64, t40772: f64, t10143: f64, t606: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t85079 = t24200 * t225;
    let t85146 = t24237 * t225;
    let t85152 = t24235 * t225;
    let t85416 = t7415 * t111;
    let t85423 = t24954 * t112;
    let t85428 = t24542 * t111;
    let t86716 = t40772 * t25;
    let t86770 = t10143 * t606;
    (t85079, t85146, t85152, t85416, t85423, t85428, t86716, t86770)
}
