//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta465 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1438;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1439;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta465(t17306: f64, t3754: f64, t10308: f64, t1466: f64, t2246: f64, t5812: f64, t11064: f64, t6075: f64, t37: f64, t5940: f64, t2609: f64, t5825: f64, t706: f64, t2611: f64, t5819: f64, t14440: f64, t4311: f64, t123: f64, t2630: f64, t5941: f64, t18555: f64, t2619: f64, t18562: f64, t2516: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t60019, t60224, t60673, t61033, t61037, t61090) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1438(t17306, t3754, t10308, t1466, t2246, t5812, t11064, t6075, t37, t5940, t2609, t5825, t706);
        let (t61165, t61180, t61247, t61282, t61294) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1439(t2609, t2611, t5819, t14440, t4311, t123, t2630, t5941, t18555, t2619, t18562, t2516);
    (t60019, t60224, t60673, t61033, t61037, t61090, t61165, t61180, t61247, t61282, t61294)
}
