//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1244/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1244(t15030: f64, t15785: f64, t1241: f64, t1251: f64, t5088: f64, t3598: f64, t1760: f64, t3599: f64, t11606: f64, t225: f64, t4941: f64, t1751: f64, t3481: f64) -> (f64, f64, f64, f64, f64) {
    let t15786 = t15030 + t15785;
    let t15787 = t1241 * t15786;
    let t15789 = t5088 * t1251;
    let t15790 = t3598 * t15789;
    let t15793 = t1760 * t3599;
    let t15794 = t11606 * t15793;
    let t15797 = t4941 * t225;
    let t15800 = t3481 * t1751;
    (t15787, t15790, t15794, t15797, t15800)
}
