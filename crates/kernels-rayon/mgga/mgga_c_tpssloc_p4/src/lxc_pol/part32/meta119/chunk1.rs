//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 702/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk702(t2904: f64, t315: f64, t2764: f64, t2822: f64, t941: f64) -> (f64, f64, f64, f64, f64) {
    let t2905 = t315 * t2904;
    let t2912 = 0.40256666666666666667e0_f64 * t2764;
    let t2919 = 0.137975e0_f64 * t2822;
    let t2928 = t941 * t941;
    let t2929 = 1.0_f64 / t2928;
    (t2905, t2912, t2919, t2928, t2929)
}
