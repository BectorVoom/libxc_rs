//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 829/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk829(t1961: f64, t3814: f64, t3767: f64, t1897: f64, t3780: f64, t3762: f64, t11633: f64, t518: f64, t5481: f64, t1419: f64, t3786: f64, t3841: f64, t5463: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t16017 = t3814 * t1961;
    let t16018 = t16017 * t3767;
    let t16021 = t3780 * t1897;
    let t16022 = t16021 * t3762;
    let t16025 = t11633 * t1897;
    let t16026 = t16025 * t3767;
    let t16029 = t518 * t5481;
    let t16030 = t16029 * t1419;
    let t16031 = t3786 * t16030;
    let t16034 = t5463 * t3841;
    (t16018, t16022, t16026, t16030, t16031, t16034)
}
