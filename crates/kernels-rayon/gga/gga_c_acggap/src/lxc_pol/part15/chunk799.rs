//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 799/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk799(t8607: f64, t8619: f64, t8625: f64, t7331: f64, t7350: f64, t7366: f64, t8133: f64, t8144: f64, t8145: f64, t8146: f64, t8598: f64, t8603: f64, t8611: f64, t8615: f64, t8623: f64) -> f64 {
    let t9222 = 0.42874018118069736972e-3_f64 * t8607;
    let t9226 = 0.28015625e-1_f64 * t8619;
    let t9228 = 7.0_f64 / 144.0_f64 * t8625;
    let t9229 = -t8133 + t7331 + 0.18868855373762491241e-2_f64 * t8598 - 0.37737710747524982483e-2_f64 * t8603 + t9222 + 0.21437009059034868486e-2_f64 * t8611 + 0.12862205435420921092e-2_f64 * t8615 + t7350 - 0.31448092289604152069e-3_f64 * t7366 + t8144 - t8145 + t8146 + t9226 - t8623 / 192.0_f64 + t9228;
    t9229
}
