//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1150/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1150(t1058: f64, t1060: f64, t2207: f64, t7088: f64, t3308: f64, t37961: f64, t7368: f64, t10776: f64, t7429: f64, t10781: f64, t7505: f64, t11837: f64, t1584: f64) -> (f64, f64, f64, f64, f64) {
    let t40011 = t2207 * t1058 * t1060 * t7088;
    let t40016 = t37961 * t3308 * t7368;
    let t40019 = t10776 * t3308 * t7429;
    let t40021 = t10781 * t7505;
    let t40024 = t1584 * t11837;
    (t40011, t40016, t40019, t40021, t40024)
}
