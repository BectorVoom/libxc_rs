//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 998/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk998(t1988: f64, t22716: f64, t22724: f64, t6898: f64, t225: f64, t3886: f64, t25: f64, t2752: f64, t1887: f64, t6581: f64) -> (f64, f64, f64, f64, f64) {
    let t22923 = t22716 * t1988;
    let t22925 = t22724 * t6898;
    let t22933 = t225 * t3886;
    let t22960 = t2752 * t25;
    let t22986 = t6581 * t1887;
    (t22923, t22925, t22933, t22960, t22986)
}
