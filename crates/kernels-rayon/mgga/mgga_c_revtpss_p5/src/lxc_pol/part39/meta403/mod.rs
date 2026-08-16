//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta403 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1475;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1476;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1477;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta403(t101: f64, t613: f64, t655: f64, t100: f64, t43: f64, t658: f64, t2349: f64, t96: f64, t2350: f64, t2256: f64, t8268: f64, t31026: f64, t31028: f64, t31030: f64, t31033: f64, t31035: f64, t31036: f64, t31040: f64, t31044: f64, t31047: f64, t69: f64, t8258: f64, t8267: f64, t114: f64, t569: f64, t1453: f64, t8273: f64, t508: f64, t2178: f64, t4151: f64, t10416: f64, t1312: f64, t13435: f64, t13440: f64, t18163: f64, t2179: f64, t2181: f64, t2322: f64, t31013: f64, t31016: f64, t4254: f64, t5523: f64, t651: f64, t8254: f64, t8274: f64, t8278: f64, t8280: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t31051, t31054, t31055, t31058, t31059, t31062, t31065) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1475(t101, t613, t655, t100, t43, t658, t2349, t96, t2350, t2256, t8268, t31026, t31028, t31030, t31033, t31035, t31036, t31040, t31044, t31047, t69, t8258, t8267);
        let t31066 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1476(t114, t31065);
        let (t31067, t31070, t31073, t31084, t31087) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1477(t31066, t569, t1453, t8273, t508, t2178, t4151, t10416, t1312, t13435, t13440, t18163, t2179, t2181, t2322, t31013, t31016, t4254, t5523, t651, t8254, t8274, t8278, t8280);
    (t31051, t31054, t31055, t31058, t31059, t31062, t31066, t31067, t31070, t31073, t31084, t31087)
}
