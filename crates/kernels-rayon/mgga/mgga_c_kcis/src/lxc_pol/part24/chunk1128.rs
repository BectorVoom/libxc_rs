//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1128/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1128(t1008: f64, t71722: f64, t1262: f64, t6276: f64, t167: f64, t1851: f64, t26391: f64, t26399: f64, t26401: f64, t26409: f64, t26655: f64, t26520: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t81752 = t71722 * t1008;
    let t84759 = t6276 * t1262;
    let t84812 = t1851 * t167;
    let t91769 = 18.0_f64 * t26391;
    let t91772 = 6.0_f64 * t26399;
    let t91773 = 12.0_f64 * t26401;
    let t91776 = 6.0_f64 * t26409;
    let t91777 = 3.0_f64 * t26655;
    let t91778 = 3.0_f64 * t26520;
    (t81752, t84759, t84812, t91769, t91772, t91773, t91776, t91777, t91778)
}
