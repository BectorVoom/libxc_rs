//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2760/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2760(t22115: f64, t9962: f64, t13999: f64, t22163: f64, t22048: f64, t22089: f64, t22076: f64, t6861: f64, t9994: f64, t1398: f64, t125: f64, t22252: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t73805 = t9962 * t22115;
    let t73811 = t13999 * t22163;
    let t73813 = t13999 * t22048;
    let t73815 = t13999 * t22089;
    let t73818 = t9962 * t22076;
    let t73820 = t6861 * t9994;
    let t73842 = t6861 * t1398;
    let t73847 = t125 * t22252;
    (t73805, t73811, t73813, t73815, t73818, t73820, t73842, t73847)
}
