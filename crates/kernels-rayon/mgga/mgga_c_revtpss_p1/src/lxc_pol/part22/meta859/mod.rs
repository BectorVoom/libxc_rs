//! MGGA_C_REVTPSS lxc pol kernel — _part22_v4rho4_2 meta859 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3007;
use chunk1::mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3008;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_meta859(t10696: f64, t1544: f64, t14832: f64, t2394: f64, t2661: f64, t14668: f64, t14923: f64, t124: f64, t4423: f64, t14686: f64, t14931: f64, t4366: f64, t2645: f64, t2722: f64, t1558: f64, t231: f64, t40406: f64, t685: f64, t72: f64, t826: f64, t14869: f64, t9775: f64, t10899: f64, t136: f64, t216: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t50399, t50409, t50412, t50415) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3007(t10696, t1544, t14832, t2394, t2661, t14668, t14923, t124, t4423, t14686, t14931, t4366);
        let (t50418, t50423, t50436, t50443, t50446) = mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3008(t1544, t2645, t2722, t1558, t231, t40406, t685, t72, t826, t14869, t9775, t10899, t136, t216);
    (t50399, t50409, t50412, t50415, t50418, t50423, t50436, t50443, t50446)
}
