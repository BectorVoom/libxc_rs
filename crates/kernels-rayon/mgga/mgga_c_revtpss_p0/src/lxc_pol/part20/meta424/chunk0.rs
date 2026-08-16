//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1591/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1591(t406: f64, t43822: f64, t12254: f64, t141: f64, t43835: f64, t1145: f64, t43843: f64, t1139: f64, t43908: f64, t3407: f64, t43825: f64, t43886: f64, t43888: f64, t43890: f64, t43892: f64, t43894: f64, t43896: f64, t43899: f64, t43902: f64, t43905: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t43946 = f64::powf(t406, -0.25e1_f64);
    let t43947 = t43946 * t43822;
    let t43950 = t141 * t12254 * t43835;
    let t43953 = t141 * t1145 * t43843;
    let t43955 = t1139 * t43908;
    let t43957 = t3407 * t43825;
    let t43959 = 0.40256666666666666666e1_f64 * t43886 - 0.12524296296296296297e1_f64 * t43888 + 0.80513333333333333336e0_f64 * t43890 + 0.16102666666666666667e1_f64 * t43892 - 0.24154e1_f64 * t43894 - 0.40256666666666666668e0_f64 * t43896 - 0.72462e1_f64 * t43899 + 0.72462e1_f64 * t43902 + 0.301925e0_f64 * t43905 + 0.6189328125e-1_f64 * t43947 + 0.22076e0_f64 * t43950 + 0.298026e1_f64 * t43953 + 0.16504875e0_f64 * t43955 + 0.247573125e0_f64 * t43957;
    (t43947, t43950, t43953, t43955, t43957, t43959)
}
