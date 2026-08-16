//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 945/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk945(t22563: f64, t39: f64, t7837: f64, t5551: f64, t79931: f64, t373: f64, t5555: f64, t173: f64, t32266: f64, t32267: f64, t32268: f64, t1669: f64, t6: f64, t92920: f64) -> (f64, f64, f64, f64, f64) {
    let t136692 = t7837 * t22563 * t39;
    let t136693 = t79931 * t5551;
    let t136694 = t5555 * t373;
    let t136714 = t32266 * t32267 * t173 * t32268;
    let t136720 = t1669 * t92920 * t6;
    (t136692, t136693, t136694, t136714, t136720)
}
