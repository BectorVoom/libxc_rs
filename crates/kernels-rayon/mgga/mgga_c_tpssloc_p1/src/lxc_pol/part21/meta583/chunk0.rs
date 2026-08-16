//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2311/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2311(t19676: f64, t19679: f64, t19688: f64, t19699: f64, t225: f64, t1819: f64, t68: f64, t1995: f64, t6330: f64, t1307: f64, t5187: f64, t5279: f64) -> (f64, f64, f64, f64, f64) {
    let t19702 = (t19676 + t19679 + t19688 + t19699) * t225;
    let t19708 = t1819 * t68;
    let t19715 = t1995 * t6330;
    let t19716 = t19715 * t1307;
    let t19719 = t5279 * t5187;
    (t19702, t19708, t19715, t19716, t19719)
}
