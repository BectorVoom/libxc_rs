//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 2036/2049 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk2036(t100993: f64, t100997: f64, t101035: f64, t101040: f64, t101051: f64, t101061: f64, t101070: f64, t101074: f64, t103554: f64, t103561: f64, t1113: f64, t1940: f64, t2071: f64, t2403: f64, t25752: f64, t25767: f64, t26425: f64, t26590: f64, t27773: f64, t28291: f64, t28456: f64, t33: f64, t4541: f64, t51780: f64, t7428: f64, t8020: f64, t8046: f64) -> f64 {
    let t103817 = 3.0_f64 * t4541 * t2071 * t101070 - 3.0_f64 * t26425 * t101061 + t1940 * t26590 * t101040 + 3.0_f64 / 2.0_f64 * t2403 * t8020 * t25767 - t103561 + 3.0_f64 * t4541 * t8020 * t25752 + 3.0_f64 * t28291 * t101035 + 3.0_f64 * t2403 * t7428 * t27773 + t1940 * t103554 * t33 / 2.0_f64 + 3.0_f64 / 2.0_f64 * t2403 * t2071 * t100993 + 3.0_f64 * t2403 * t2071 * t100997 + 3.0_f64 * t51780 * t8046 + 3.0_f64 / 2.0_f64 * t2403 * t2071 * t101051 + 3.0_f64 / 2.0_f64 * t2403 * t2071 * t101074 + t1940 * t28456 * t1113;
    t103817
}
