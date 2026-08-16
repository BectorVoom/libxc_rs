//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta475 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1927;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta475(t1592: f64, t999: f64, t1045: f64, t15691: f64, t1066: f64, t18946: f64, t247: f64, t11725: f64, t6092: f64, t1063: f64, t3109: f64, t6100: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
        let (t20038, t20039, t20040, t20046, t20050, t20051, t20054) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1927(t1592, t999, t1045, t15691, t1066, t18946, t247, t11725, t6092, t1063, t3109, t6100);
    (t20038, t20039, t20040, t20046, t20050, t20051, t20054)
}
