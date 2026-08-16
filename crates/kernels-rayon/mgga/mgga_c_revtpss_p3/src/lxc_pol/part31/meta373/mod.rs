//! MGGA_C_REVTPSS lxc pol kernel — _part31_v4rho3sigma_6 meta373 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1410;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_meta373(t1626: f64, t3011: f64, t15125: f64, t15191: f64, t4644: f64, t945: f64, t1614: f64, t2967: f64, t2986: f64, t4587: f64, t914: f64, t1596: f64, t2923: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t15350, t15363, t15364, t15400, t15406, t15413, t15416, t15421) = mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1410(t1626, t3011, t15125, t15191, t4644, t945, t1614, t2967, t2986, t4587, t914, t1596, t2923);
    (t15350, t15363, t15364, t15400, t15406, t15413, t15416, t15421)
}
