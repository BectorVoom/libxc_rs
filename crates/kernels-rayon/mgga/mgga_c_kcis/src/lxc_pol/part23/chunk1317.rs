//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1317/1323 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1317(t1299: f64, t1885: f64, t2233: f64, t27716: f64, t446: f64, t449: f64, t6260: f64, t91791: f64, t91793: f64, t91863: f64, t91866: f64, t91869: f64, t91872: f64, t91874: f64, t97548: f64, t99737: f64, t99738: f64) -> f64 {
    let t99743 = -t91791 - t91793 - t91863 + t91866 - t446 * t1885 * t27716 / 16.0_f64 - t91869 + t91872 - t91874 + t97548 - t2233 * t1299 * t6260 / 8.0_f64 - t446 * t449 * (t99737 + t99738) / 16.0_f64;
    t99743
}
