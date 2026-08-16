//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1054/1189 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1054(t145335: f64, t22819: f64, t7195: f64, t115567: f64, t136560: f64, t136561: f64, t136692: f64, t136693: f64, t136694: f64, t136740: f64, t136885: f64, t136893: f64, t136899: f64, t136952: f64, t136988: f64, t145491: f64, t22736: f64, t25774: f64, t34421: f64, t399: f64, t930: f64) -> f64 {
    let t145536 = t22819 * t7195 * t145335;
    let t145553 = -0.11854761295685025975e-1_f64 * t34421 * t399 - 0.22705522127871165896e-3_f64 * t145536 + 0.24511020009968991682e-5_f64 * t136692 * t136693 * t136694 * t930 + t136885 - t136893 - 0.90822088511484663584e-3_f64 * t136899 - 0.13200366700519885118e-5_f64 * t136560 * t136561 * t145491 - 0.13200366700519885118e-5_f64 * t136560 * t136561 * t115567 + 0.3827206426927081041e-8_f64 * t22736 * t136740 * t25774 - 0.11738898233082762228e-1_f64 * t136952 - t136988;
    t145553
}
