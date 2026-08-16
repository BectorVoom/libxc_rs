//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta505 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1513;
use chunk1::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1514;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta505(t23263: f64, t40864: f64, t10697: f64, t23114: f64, t236: f64, t807: f64, t23267: f64, t2703: f64, t23148: f64, t854: f64, t1559: f64, t18599: f64, t2661: f64, t2662: f64, t221: f64, t23177: f64, t2484: f64, t2485: f64, t1469: f64, t4401: f64, t61303: f64, t14613: f64, t18539: f64, t18544: f64, t4311: f64, t23214: f64, t750: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t76835, t76856, t76858, t76878, t76882) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1513(t23263, t40864, t10697, t23114, t236, t807, t23267, t2703, t23148, t854, t1559, t18599, t2661, t2662);
        let (t76887, t76892, t76947, t76949, t76951) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1514(t221, t23177, t2484, t2485, t1469, t4401, t61303, t14613, t18539, t18544, t4311, t23214, t750);
    (t76835, t76856, t76858, t76878, t76882, t76887, t76892, t76947, t76949, t76951)
}
