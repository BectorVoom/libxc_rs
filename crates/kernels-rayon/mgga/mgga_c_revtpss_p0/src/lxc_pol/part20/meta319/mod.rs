//! MGGA_C_REVTPSS lxc pol kernel — _part20_v4rho4_0 meta319 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1229;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_meta319(t126: f64, t3617: f64, t247: f64, t3363: f64, t1261: f64, t12690: f64, t225: f64, t480: f64, t1231: f64, t3655: f64, t1256: f64, t3651: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t12884, t12886, t12887, t12889, t12890, t12893, t12895) = mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1229(t126, t3617, t247, t3363, t1261, t12690, t225, t480, t1231, t3655, t1256, t3651);
    (t12884, t12886, t12887, t12889, t12890, t12893, t12895)
}
