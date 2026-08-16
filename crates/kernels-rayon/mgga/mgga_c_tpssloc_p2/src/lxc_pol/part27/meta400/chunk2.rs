//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1665/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1665(t1720: f64, t3590: f64, t15425: f64, t491: f64, t1235: f64, t4940: f64, t225: f64, t5053: f64, t1190: f64, t5052: f64, t15771: f64, t466: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t15808 = t1720 * t3590;
    let t15814 = t15425 * t491;
    let t15816 = t4940 * t1235;
    let t15820 = t5053 * t225;
    let t15823 = t1190 * t5052;
    let t15831 = t466 * t15771;
    (t15808, t15814, t15816, t15820, t15823, t15831)
}
