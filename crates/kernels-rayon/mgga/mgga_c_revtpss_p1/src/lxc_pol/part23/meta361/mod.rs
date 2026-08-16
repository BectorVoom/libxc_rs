//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta361 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1675;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta361(t1569: f64, t2453: f64, t2458: f64, t4321: f64, t887: f64, t689: f64, t4470: f64, t786: f64, t789: f64, t4534: f64, t779: f64, t2435: f64, t4322: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15017, t15018, t15045, t15047, t15048, t15050, t15060, t15062, t15063) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1675(t1569, t2453, t2458, t4321, t887, t689, t4470, t786, t789, t4534, t779, t2435, t4322);
    (t15017, t15018, t15045, t15047, t15048, t15050, t15060, t15062, t15063)
}
