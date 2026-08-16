//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 800/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk800(t5025: f64, t8143: f64, t2812: f64, t2367: f64, t4933: f64, t930: f64, t4929: f64, t5016: f64, t924: f64, t2778: f64, t5011: f64, t2773: f64) -> (f64, f64, f64, f64, f64) {
    let t14669 = t8143 * t5025;
    let t14670 = t2812 * t14669;
    let t14738 = t2367 * t4933;
    let t14739 = t930 * t14738;
    let t14743 = t2367 * t4929;
    let t14744 = t930 * t14743;
    let t14752 = t924 * t5016;
    let t14753 = t2778 * t14752;
    let t14757 = t924 * t5011;
    let t14758 = t2773 * t14757;
    (t14670, t14739, t14744, t14753, t14758)
}
