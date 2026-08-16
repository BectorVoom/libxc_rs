//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 957/1173 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk957(t1701: f64, t1702: f64, t8932: f64, t2035: f64, t538: f64, t8807: f64, t554: f64, t6: f64, t8908: f64, t133: f64, t8909: f64, t542: f64, t7334: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t39835 = t1701 * t1702 * t8932;
    let t39839 = t2035 * t8807 * t538;
    let t39843 = t2035 * t8807 * t554;
    let t39846 = t8908 * t6;
    let t39847 = t133 * t39846;
    let t39849 = t1701 * t1702 * t8909;
    let t39852 = t542 * t7334;
    (t39835, t39839, t39843, t39846, t39847, t39849, t39852)
}
