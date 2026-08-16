//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 1601/2270 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk1601(t4186: f64, t999: f64, t4872: f64, t1042: f64, t4866: f64, t73: f64, t3095: f64, t3092: f64, t2857: f64, t357: f64, t2251: f64, t4781: f64) -> (f64, f64, f64, f64) {
    let t15950 = t4186 * t999;
    let t15951 = t4872 * t15950;
    let t15952 = t1042 * t15951;
    let t15957 = t4866 * t73;
    let t15958 = t15957 * t3095;
    let t15959 = t3092 * t15958;
    let t15962 = t357 * t2857;
    let t15963 = t15962 * t2251;
    let t15964 = t4781 * t15963;
    (t15952, t15957, t15959, t15964)
}
