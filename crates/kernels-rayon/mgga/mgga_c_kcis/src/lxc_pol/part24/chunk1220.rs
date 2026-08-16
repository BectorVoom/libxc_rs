//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1220/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1220(t7755: f64, t99916: f64, t2825: f64, t6728: f64, t6720: f64, t92437: f64, t20205: f64, t26930: f64, t3178: f64, t6724: f64, t6732: f64, t19953: f64, t26896: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t99917 = t99916 * t7755;
    let t99919 = t2825 * t6728;
    let t99921 = t92437 * t6720;
    let t99923 = t26930 * t20205;
    let t99925 = t3178 * t6724;
    let t99927 = t2825 * t6732;
    let t99929 = t26896 * t19953;
    (t99917, t99919, t99921, t99923, t99925, t99927, t99929)
}
