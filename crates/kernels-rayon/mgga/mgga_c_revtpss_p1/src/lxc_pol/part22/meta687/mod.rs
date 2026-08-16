//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta687 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2677;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2678;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta687(t30: f64, t21881: f64, t508: f64, t1518: f64, t5517: f64, t13584: f64, t9375: f64, t6785: f64, t9335: f64, t3833: f64, t5824: f64, t18280: f64, t2255: f64, t513: f64, t5549: f64, t605: f64, zeta_threshold: f64, t33: f64, t6792: f64, t9350: f64, t3841: f64, t6416: f64, t1113: f64, t20256: f64, t516: f64, t5557: f64, t162: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t21882, t21891, t21901, t21905, t21906, t21911, t21917) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2677(t30, t21881, t508, t1518, t5517, t13584, t9375, t6785, t9335, t3833, t5824, t18280, t2255, t513, t5549, t605, zeta_threshold);
        let (t21918, t21923, t21931) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2678(t33, t6792, t9350, t3841, t6416, t1113, t20256, t2255, t516, t5557, t162, t21917, zeta_threshold);
    (t21882, t21891, t21901, t21905, t21906, t21911, t21918, t21923, t21931)
}
