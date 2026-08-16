//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta802 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2630;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2631;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta802(t23160: f64, t836: f64, t10529: f64, t2782: f64, t14520: f64, t14606: f64, t6016: f64, t860: f64, t231: f64, t2783: f64, t18657: f64, t686: f64, t72: f64, t874: f64, t1559: f64, t4423: f64, t2797: f64, t14586: f64, t18725: f64, t2470: f64, t2798: f64, t10542: f64, t18730: f64, t61749: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t62606, t62609, t62612, t62615, t62619) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2630(t23160, t836, t10529, t2782, t14520, t14606, t6016, t860, t231, t2783, t18657, t686, t72, t874);
        let (t62626, t62630, t62633, t62635, t62637) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2631(t1559, t4423, t2782, t2797, t14586, t10529, t18725, t2470, t2798, t10542, t18730, t231, t61749);
    (t62606, t62609, t62612, t62615, t62619, t62626, t62630, t62633, t62635, t62637)
}
