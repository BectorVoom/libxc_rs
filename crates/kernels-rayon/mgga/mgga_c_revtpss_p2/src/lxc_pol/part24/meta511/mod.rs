//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta511 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1526;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1527;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta511(t1063: f64, t11725: f64, t23481: f64, t247: f64, t23474: f64, t3109: f64, t23847: f64, t3127: f64, t3172: f64, t23858: f64, t23634: f64, t1065: f64, t24031: f64, t11256: f64, t23642: f64, t23811: f64, t300: f64, t23470: f64, t42534: f64, t20050: f64, t4834: f64, t23843: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t78550, t78561, t78564, t78576, t78583, t78607) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1526(t1063, t11725, t23481, t247, t23474, t3109, t23847, t3127, t3172, t23858, t23634, t1065, t24031);
        let (t78676, t78704, t78750, t78756, t78763) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1527(t11256, t23642, t3172, t23811, t300, t1063, t23470, t247, t42534, t20050, t4834, t23843);
    (t78550, t78561, t78564, t78576, t78583, t78607, t78676, t78704, t78750, t78756, t78763)
}
