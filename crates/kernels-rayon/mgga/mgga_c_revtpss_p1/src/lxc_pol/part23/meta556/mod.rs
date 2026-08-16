//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta556 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2114;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2115;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta556(t1357: f64, t6919: f64, t689: f64, t1444: f64, t6918: f64, t4076: f64, t14081: f64, t14084: f64, t14087: f64, t1424: f64, t14299: f64, t1904: f64, t22395: f64, t22400: f64, t22405: f64, t22407: f64, t9677: f64, t9687: f64, t9691: f64, t5599: f64, t10157: f64, t14091: f64, t14096: f64, t14097: f64, t14102: f64, t14105: f64, t14108: f64, t14111: f64, t14276: f64, t5715: f64, t5728: f64, t9694: f64, t9695: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t22409, t22410, t22414, t22415, t22418) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2114(t1357, t6919, t689, t1444, t6918, t4076, t14081, t14084, t14087, t1424, t14299, t1904, t22395, t22400, t22405, t22407, t9677, t9687, t9691);
        let (t22427, t22428, t22430) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2115(t1904, t5599, t689, t10157, t14091, t14096, t14097, t14102, t14105, t14108, t14111, t14276, t5715, t5728, t9694, t9695);
    (t22409, t22410, t22414, t22415, t22418, t22427, t22428, t22430)
}
