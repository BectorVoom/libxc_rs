//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1865/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1865(t4631: f64, t4635: f64, t2924: f64, t11404: f64, t11548: f64, t15400: f64, t1622: f64, t19046: f64, t19079: f64, t19130: f64, t19132: f64, t19173: f64, t19227: f64, t19247: f64, t2938: f64, t311: f64, t4647: f64, t4670: f64, t6158: f64, t6174: f64, t6177: f64, t946: f64, t955: f64) -> (f64, f64, f64) {
    let t19250 = t4635 * t4631;
    let t19252 = 0.32163958997385070134e2_f64 * t2924 * t19250;
    let t19253 = t19079 - t19130 - t19132 + 1.0_f64 * t19173 * t955 + 2.0_f64 * t15400 * t1622 + 2.0_f64 * t4647 * t4670 - 2.0_f64 * t11548 * t6158 + 1.0_f64 * t2938 * t6174 + 1.0_f64 * t946 * t19227 + 0.32163958997385070134e2_f64 * t11404 * t6177 - 0.19751673498613801407e-1_f64 * t19046 - 0.310907e-1_f64 * t19247 * t311 - t19252;
    (t19250, t19252, t19253)
}
