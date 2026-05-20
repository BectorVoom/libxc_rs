//! MGGA_C_REVTPSS lxc pol kernel — _part39_v4rho3tau_2 meta403 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1475;
use chunk1::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1476;
use chunk2::mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1477;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_meta403<F: Float>(t101: F, t613: F, t655: F, t100: F, t43: F, t658: F, t2349: F, t96: F, t2350: F, t2256: F, t8268: F, t31026: F, t31028: F, t31030: F, t31033: F, t31035: F, t31036: F, t31040: F, t31044: F, t31047: F, t69: F, t8258: F, t8267: F, t114: F, t569: F, t1453: F, t8273: F, t508: F, t2178: F, t4151: F, t10416: F, t1312: F, t13435: F, t13440: F, t18163: F, t2179: F, t2181: F, t2322: F, t31013: F, t31016: F, t4254: F, t5523: F, t651: F, t8254: F, t8274: F, t8278: F, t8280: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
        let (t31051, t31054, t31055, t31058, t31059, t31062, t31065) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1475::<F>(t101, t613, t655, t100, t43, t658, t2349, t96, t2350, t2256, t8268, t31026, t31028, t31030, t31033, t31035, t31036, t31040, t31044, t31047, t69, t8258, t8267);
        let t31066 = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1476::<F>(t114, t31065);
        let (t31067, t31070, t31073, t31084, t31087) = mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1477::<F>(t31066, t569, t1453, t8273, t508, t2178, t4151, t10416, t1312, t13435, t13440, t18163, t2179, t2181, t2322, t31013, t31016, t4254, t5523, t651, t8254, t8274, t8278, t8280);
    (t31051, t31054, t31055, t31058, t31059, t31062, t31066, t31067, t31070, t31073, t31084, t31087)
}
