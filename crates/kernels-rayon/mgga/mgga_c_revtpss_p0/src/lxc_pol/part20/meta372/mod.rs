//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta372 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1352;
use chunk1::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1353;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta372(t10867: f64, t860: f64, t2722: f64, t2723: f64, t10069: f64, t10929: f64, t138: f64, t785: f64, t9302: f64, t2786: f64, t10073: f64, t10920: f64, t231: f64, t2760: f64, t2782: f64, t2783: f64, t836: f64, t10871: f64, t14545: f64, t39709: f64, t2645: f64, t234: f64, t39545: f64, t685: f64, t875: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t40258, t40262, t40263, t40267, t40270, t40271, t40273) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1352(t10867, t860, t2722, t2723, t10069, t10929, t138, t785, t9302, t2786, t10073, t10920);
        let (t40278, t40282, t40284, t40294) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1353(t231, t2760, t2782, t2783, t836, t10871, t14545, t39709, t2645, t234, t39545, t685, t875);
    (t40258, t40262, t40263, t40267, t40270, t40271, t40273, t40278, t40282, t40284, t40294)
}
