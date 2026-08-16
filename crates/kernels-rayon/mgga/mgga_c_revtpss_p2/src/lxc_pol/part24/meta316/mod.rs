//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta316 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1103;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta316(t1432: f64, t22379: f64, t686: f64, t213: f64, t6888: f64, t6918: f64, t72: f64, t3915: f64, t6889: f64, t786: f64, t1364: f64, t14100: f64, t5722: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22381, t22390, t22398, t22399, t22400, t22404, t22405, t22407) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1103(t1432, t22379, t686, t213, t6888, t6918, t72, t3915, t6889, t786, t1364, t14100, t5722);
    (t22381, t22390, t22398, t22399, t22400, t22404, t22405, t22407)
}
