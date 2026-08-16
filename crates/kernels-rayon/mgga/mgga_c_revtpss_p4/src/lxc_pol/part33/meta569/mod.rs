//! MGGA_C_REVTPSS lxc pol kernel — _part33_v4rho3sigma_8 meta569 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;
mod chunk2;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1976;
use chunk1::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1977;
use chunk2::mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1978;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_meta569(t1518: f64, t8233: f64, t1519: f64, t2165: f64, t29427: f64, t29590: f64, t29993: f64, t29998: f64, t30007: f64, t30015: f64, t30113: f64, t30125: f64, t30127: f64, t30130: f64, t30154: f64, t30156: f64, t30158: f64, t30951: f64, t30959: f64, t4248: f64, t569: f64, t5887: f64, t5921: f64, t651: f64, t6934: f64, t7586: f64, t8158: f64, t30950: f64, t3: f64, t1918: f64, t2170: f64, t30180: f64, t30182: f64, t30184: f64, t30187: f64, t30190: f64, t30193: f64, t30196: f64, t573: f64, t6945: f64, t6948: f64, t8245: f64, param_d: f64, t4147: f64, t7311: f64, t1353: f64, t2033: f64, t7933: f64, t2126: f64, t1450: f64, t11239: f64, t3736: f64, t211: f64, t9644: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t30963, t30973) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1976(t1518, t8233, t1519, t2165, t29427, t29590, t29993, t29998, t30007, t30015, t30113, t30125, t30127, t30130, t30154, t30156, t30158, t30951, t30959, t4248, t569, t5887, t5921, t651, t6934, t7586, t8158);
        let (t30974, t30975, t30985, t30993) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1977(t30950, t30973, t3, t1918, t2170, t30180, t30182, t30184, t30187, t30190, t30193, t30196, t573, t6945, t6948, t8245, param_d);
        let (t32113, t32737, t33651, t34446, t35669, t37885, t39643) = mgga_c_revtpss_lxc_pol_part33_v4rho3sigma_8_chunk1978(t4147, t7311, t1353, t2033, t7933, t1518, t2126, t1450, t11239, t3736, t211, t9644);
    (t30963, t30974, t30975, t30985, t30993, t32113, t32737, t33651, t34446, t35669, t37885, t39643)
}
