//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta370 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1260;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta370(t5192: f64, t6552: f64, t1188: f64, t24375: f64, t3520: f64, t1196: f64, t1765: f64, t20400: f64, t5197: f64, t6535: f64, t6556: f64, t12485: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t24478, t24480, t24482, t24484, t24488, t24490, t24492, t24493) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1260(t5192, t6552, t1188, t24375, t3520, t1196, t1765, t20400, t5197, t6535, t6556, t12485);
    (t24478, t24480, t24482, t24484, t24488, t24490, t24492, t24493)
}
