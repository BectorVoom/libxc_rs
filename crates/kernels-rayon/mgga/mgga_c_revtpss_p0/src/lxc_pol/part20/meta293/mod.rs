//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta293 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1164;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta293(t12234: f64, t3516: f64, t1196: f64, t1130: f64, t3376: f64, t1151: f64, t3379: f64, t3428: f64, t1126: f64, t3432: f64, t3436: f64, t3431: f64, t418: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t12235, t12237, t12238, t12240, t12242, t12243, t12245, t12247) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1164(t12234, t3516, t1196, t1130, t3376, t1151, t3379, t3428, t1126, t3432, t3436, t3431, t418);
    (t12235, t12237, t12238, t12240, t12242, t12243, t12245, t12247)
}
