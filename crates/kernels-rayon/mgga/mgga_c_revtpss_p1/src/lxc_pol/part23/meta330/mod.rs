//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta330 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1629;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta330(t14047: f64, t2661: f64, t1399: f64, t5608: f64, t3992: f64, t5651: f64, t5774: f64, t72: f64, t686: f64, t3915: f64, t5711: f64, t786: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t14049, t14051, t14053, t14055, t14057, t14078, t14079, t14081, t14082) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1629(t14047, t2661, t1399, t5608, t3992, t5651, t5774, t72, t686, t3915, t5711, t786);
    (t14049, t14051, t14053, t14055, t14057, t14078, t14079, t14081, t14082)
}
