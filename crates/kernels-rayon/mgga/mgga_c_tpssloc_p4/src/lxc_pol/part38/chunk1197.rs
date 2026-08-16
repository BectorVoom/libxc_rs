//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1197/1335 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1197(t1009: f64, t4940: f64, t1243: f64, t14701: f64, t14833: f64, t14835: f64, t14837: f64, t14840: f64, t14844: f64, t14847: f64, t14849: f64, t14852: f64, t14857: f64, t14860: f64, t14862: f64, t14864: f64, t14866: f64, t14916: f64, t14936: f64, t14939: f64) -> (f64, f64, f64) {
    let t15031 = t4940 * t1009;
    let t15032 = t15031 * t1243;
    let t15035 = t14701 - t14833 - t14835 - t14837 - t14840 + t14844 + t14847 + t14849 + t14852 - t14857 - t14860 - t14862 + t14864 + t14866 + t14916 + t14936 + t14939;
    (t15031, t15032, t15035)
}
