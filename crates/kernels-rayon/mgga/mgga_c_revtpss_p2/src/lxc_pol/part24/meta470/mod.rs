//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta470 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1448;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1449;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta470(t18353: f64, t2689: f64, t18348: f64, t2710: f64, t2713: f64, t18562: f64, t2626: f64, t2609: f64, t5944: f64, t10815: f64, t5980: f64, t40398: f64, t6024: f64, t10716: f64, t18423: f64, t18415: f64, t9775: f64, t18410: f64, t10995: f64, t18804: f64, t2470: f64, t18725: f64, t2798: f64, t10069: f64, t18738: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t62129, t62251, t62276, t62300, t62399, t62401) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1448(t18353, t2689, t18348, t2710, t2713, t18562, t2626, t2609, t5944, t10815, t5980, t40398, t6024);
        let (t62431, t62443, t62445, t62528, t62633, t62649) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1449(t10716, t18423, t18415, t9775, t18410, t10995, t18804, t2470, t18725, t2798, t10069, t18738);
    (t62129, t62251, t62276, t62300, t62399, t62401, t62431, t62443, t62445, t62528, t62633, t62649)
}
