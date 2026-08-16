//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta318 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1607;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta318(t2: f64, t3833: f64, t1711: f64, t9350: f64, t3841: f64, t1857: f64, t3857: f64, t177: f64, t5566: f64, t762: f64, t1450: f64, t5778: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t13553, t13565, t13568, t13584, t13597, t13599, t13600) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1607(t2, t3833, t1711, t9350, t3841, t1857, t3857, t177, t5566, t762, t1450, t5778);
    (t13553, t13565, t13568, t13584, t13597, t13599, t13600)
}
