//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 923/1253 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk923(t1065: f64, t481: f64, t3270: f64, t10667: f64, t2104: f64, t3436: f64, t2302: f64) -> (f64, f64, f64) {
    let t10668 = t1065 * t481;
    let t10669 = t3270 * t10668;
    let t10670 = t10667 * t10669;
    let t10671 = 3.0_f64 / 2.0_f64 * t10670;
    let t10672 = t2104 * t3436;
    let t10673 = t2302 * t10672;
    (t10669, t10671, t10673)
}
