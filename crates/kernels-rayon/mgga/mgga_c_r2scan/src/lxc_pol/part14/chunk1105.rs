//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 1105/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk1105(t10673: f64, t11587: f64, t37360: f64, t11584: f64, t37373: f64, t37426: f64, t37427: f64, t37428: f64, t898: f64, t10929: f64, t37434: f64, t37435: f64) -> (f64, f64, f64, f64) {
    let t39218 = t10673 * t11587 * t37360;
    let t39221 = t37373 * t11584;
    let t39225 = t37426 * t37427 * t898 * t37428;
    let t39229 = t37434 * t37435 * t898 * t10929;
    (t39218, t39221, t39225, t39229)
}
