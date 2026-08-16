//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta584 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2005;
use chunk1::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2006;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta584(t1096: f64, t4982: f64, t1981: f64, t42058: f64, t7143: f64, t11120: f64, t3140: f64, t1035: f64, t1983: f64, t3057: f64, t7135: f64, t11200: f64, t1976: f64, t3063: f64, t8521: f64, t36870: f64, t19482: f64, t27668: f64, t995: f64, t988: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t93984, t93994, t94016, t94023, t94026) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2005(t1096, t4982, t1981, t42058, t7143, t11120, t3140, t1035, t1983, t3057, t7135, t11200, t1976);
        let (t94042, t94053, t94063, t94064, t94080, t94081) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2006(t3063, t8521, t11200, t7143, t1035, t1983, t36870, t1096, t19482, t27668, t995, t4982, t988);
    (t93984, t93994, t94016, t94023, t94026, t94042, t94053, t94063, t94064, t94080, t94081)
}
