//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1236/1389 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1236(t26966: f64, t27055: f64, t26676: f64, t2822: f64, t7772: f64, t92860: f64, t27006: f64, t26763: f64, t2861: f64, t15573: f64, t27019: f64, t7788: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t92921 = t26966 * t27055;
    let t92929 = t2822 * t26676;
    let t92931 = t7772 * t92860;
    let t92941 = t26966 * t27006;
    let t92943 = t2861 * t26763;
    let t92945 = t15573 * t27019;
    let t92946 = t7788 * t92945;
    (t92921, t92929, t92931, t92941, t92943, t92945, t92946)
}
