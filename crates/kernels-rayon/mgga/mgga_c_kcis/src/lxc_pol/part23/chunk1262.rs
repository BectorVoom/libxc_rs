//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1262/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1262(t491: f64, t6019: f64, t1394: f64, t7924: f64, t28388: f64, t98137: f64, t28328: f64, t4142: f64, t7908: f64, t98364: f64, t15967: f64, t28332: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t98618 = t6019 * t491;
    let t98620 = t1394 * t98618 * t7924;
    let t98623 = 0.12378114784505208333e-4_f64 * t28388 * t98137;
    let t98624 = t4142 * t28328;
    let t98625 = 0.22109259259259259258e-2_f64 * t98624;
    let t98627 = 0.46336805555555555556e-3_f64 * t7908 * t98364;
    let t98628 = t7908 * t98137;
    let t98632 = t15967 * t28332;
    (t98620, t98623, t98624, t98625, t98627, t98628, t98632)
}
