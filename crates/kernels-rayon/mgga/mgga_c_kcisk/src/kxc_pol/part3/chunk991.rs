//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 991/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk991(t1511: f64, t492: f64, t1414: f64, t4232: f64, t14234: f64, t4231: f64, t6368: f64, t14556: f64, t14558: f64, t14561: f64, t14563: f64, t14565: f64, t14568: f64, t14571: f64, t14575: f64, t14579: f64, t14582: f64, t14584: f64, t14586: f64, t14589: f64) -> (f64, f64, f64) {
    let t14591 = t492 * t1511;
    let t14592 = t1414 * t14591;
    let t14593 = t14592 * t4232;
    let t14595 = t4231 * t14234;
    let t14596 = t6368 * t14595;
    let t14598 = 3.0_f64 / 8.0_f64 * t14556 + 11.0_f64 / 6.0_f64 * t14558 + t14561 / 36.0_f64 - t14563 / 64.0_f64 - t14565 / 2.0_f64 + t14568 / 24.0_f64 + t14571 / 9.0_f64 + t14575 / 54.0_f64 + t14579 / 256.0_f64 - t14582 / 4.0_f64 + t14584 / 16.0_f64 - 19.0_f64 / 48.0_f64 * t14586 - 11.0_f64 / 6.0_f64 * t14589 - t14593 / 6.0_f64 - t14596 / 32.0_f64;
    (t14593, t14596, t14598)
}
