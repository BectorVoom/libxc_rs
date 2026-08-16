//! MGGA_C_REVTPSS lxc pol kernel — _part24_v4rho4_4 meta248 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1011;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_meta248(t136: f64, t1568: f64, t2457: f64, t2710: f64, t2470: f64, t4522: f64, t874: f64, t2718: f64, t1569: f64, t867: f64, t786: f64, t2435: f64, t4477: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t14946, t14948, t14951, t14961, t14986, t14987, t14998) = mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1011(t136, t1568, t2457, t2710, t2470, t4522, t874, t2718, t1569, t867, t786, t2435, t4477);
    (t14946, t14948, t14951, t14961, t14986, t14987, t14998)
}
