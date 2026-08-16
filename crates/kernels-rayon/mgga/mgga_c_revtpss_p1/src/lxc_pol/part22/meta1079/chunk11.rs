//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3881/3938 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3881(t22074: f64, t3936: f64, t4004: f64, t48982: f64, t48984: f64, t49001: f64, t49003: f64, t49005: f64, t49008: f64, t49012: f64, t49016: f64, t49024: f64, t49030: f64, t5671: f64) -> f64 {
    let t74574 = -0.10841600599314203354e-2_f64 * t48982 - 0.80031500487063509015e-2_f64 * t48984 + 0.57165357490759649296e-3_f64 * t49001 - 0.12004725073059526352e-1_f64 * t49003 - 0.80031500487063509015e-2_f64 * t49005 - 0.17149607247227894789e-2_f64 * t5671 * t3936 * t22074 * t4004 - 0.72286371995927450868e-4_f64 * t49008 - 0.4065600224742826258e-4_f64 * t49012 + 0.15246000842785598467e-3_f64 * t49016 - 0.2032800112371413129e-3_f64 * t49024 + 455.0_f64 / 324.0_f64 * t49030;
    t74574
}
