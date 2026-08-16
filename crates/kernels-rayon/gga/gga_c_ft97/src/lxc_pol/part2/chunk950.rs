//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 950/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk950(t14730: f64, t1701: f64, t10363: f64, t1208: f64, t1196: f64, t2724: f64, t2726: f64, t1200: f64, t14728: f64, t2735: f64, t3780: f64, t2719: f64, t4109: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t14731 = t1701 * t14730;
    let t14734 = t10363 * t1208;
    let t14738 = t2724 * t1196;
    let t14739 = t14738 * t2726;
    let t14742 = t1200 * t14728;
    let t14745 = t3780 * t2735;
    let t14746 = t1701 * t14745;
    let t14749 = t4109 * t2719;
    (t14731, t14734, t14739, t14742, t14746, t14749)
}
