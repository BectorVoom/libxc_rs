//! MGGA_C_REVTPSS lxc pol kernel — _part23_v4rho4_3 meta722 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;
mod chunk1;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2484;
use chunk1::mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2485;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_meta722(t5618: f64, t9784: f64, t820: f64, t844: f64, t9991: f64, t13776: f64, t9775: f64, t46644: f64, t5622: f64, t5614: f64, t9779: f64, t40488: f64, t5610: f64, t2659: f64, t4086: f64, t816: f64, t1412: f64, t808: f64, t1389: f64, t14224: f64, t46835: f64, t13769: f64, t2453: f64, t547: f64, t9794: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t48833, t48836, t48848, t48849, t48851, t48853) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2484(t5618, t9784, t820, t844, t9991, t13776, t9775, t46644, t5622, t5614, t9779, t40488, t5610);
        let (t48862, t48863, t48869, t48872) = mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2485(t2659, t4086, t816, t1412, t808, t1389, t14224, t46835, t13769, t2453, t547, t9794);
    (t48833, t48836, t48848, t48849, t48851, t48853, t48862, t48863, t48869, t48872)
}
