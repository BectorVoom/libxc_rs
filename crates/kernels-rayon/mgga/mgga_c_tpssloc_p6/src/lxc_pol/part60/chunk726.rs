//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 60 (v4rho2sigma2_16) CSE chunk 726/1064 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part60_v4rho2sigma2_16_chunk726(t24811: f64, t467: f64, t3030: f64, t461: f64, t3502: f64, t1011: f64, t3508: f64, t1209: f64, t475: f64, t1193: f64, t7372: f64, t210: f64, t7371: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t24812 = t24811 * t467;
    let t24813 = t461 * t3030;
    let t24814 = t24813 * t3502;
    let t24815 = t1011 * t3508;
    let t24820 = t24813 * t1209;
    let t24821 = t1011 * t475;
    let t24826 = t7372 * t1193;
    let t24847 = t7371 * t210;
    (t24812, t24814, t24815, t24820, t24821, t24826, t24847)
}
