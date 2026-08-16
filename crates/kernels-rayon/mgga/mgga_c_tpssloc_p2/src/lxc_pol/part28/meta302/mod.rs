//! MGGA_C_TPSSLOC lxc pol kernel — _part28_v4rho3sigma_4 meta302 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1220;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_meta302(t10544: f64, t2784: f64, t892: f64, t2841: f64, t888: f64, t2840: f64, t287: f64, t275: f64, t10294: f64, t891: f64, t2843: f64, t290: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t10636, t10650, t10655, t10661, t10675, t10676, t10702, t10704) = mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1220(t10544, t2784, t892, t2841, t888, t2840, t287, t275, t10294, t891, t2843, t290);
    (t10636, t10650, t10655, t10661, t10675, t10676, t10702, t10704)
}
