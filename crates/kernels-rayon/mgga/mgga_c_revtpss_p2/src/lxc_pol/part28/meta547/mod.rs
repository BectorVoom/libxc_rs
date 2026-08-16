//! MGGA_C_REVTPSS lxc pol kernel — _part28_v4rho3sigma_3 meta547 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1996;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_meta547(t231: f64, t50511: f64, t198: f64, t2394: f64, t11821: f64, t65: f64, t2246: f64, t4171: f64, t10308: f64, t1466: f64, t13267: f64, t602: f64) -> (f64, f64, f64, f64, f64, f64) {
        let (t51698, t51780, t53321, t60221, t60224, t60248) = mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk1996(t231, t50511, t198, t2394, t11821, t65, t2246, t4171, t10308, t1466, t13267, t602);
    (t51698, t51780, t53321, t60221, t60224, t60248)
}
