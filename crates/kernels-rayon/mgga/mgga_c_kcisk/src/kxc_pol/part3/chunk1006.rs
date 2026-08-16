//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1006/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1006(t12902: f64, t12904: f64, t12907: f64, t12914: f64, t13018: f64, t13026: f64, t13039: f64, t13050: f64, t13053: f64, t13056: f64, t13060: f64, t13123: f64, t14817: f64, t14821: f64, t14824: f64, t14828: f64, t14842: f64, t4436: f64, t4461: f64, t4471: f64, t4478: f64, t516: f64) -> f64 {
    let t14846 = -6.0_f64 * t4436 * t14817 + 0.96494049533612093922e2_f64 * t4461 * t14821 - 0.35089340384731224426e1_f64 * t4471 * t14824 + 0.51947267698127589897e2_f64 * t4478 * t14828 + t13123 + t13056 - t13060 - t12902 - 0.3109e-1_f64 * t14842 * t516 - 0.19751789702565206229e-1_f64 * t13039 - t12904 - t12907 + t12914 - t13018 - t13026 + t13050 - t13053;
    t14846
}
