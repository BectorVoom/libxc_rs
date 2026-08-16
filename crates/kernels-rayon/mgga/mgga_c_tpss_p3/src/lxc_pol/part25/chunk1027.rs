//! MGGA_C_TPSS lxc pol — lxc_pol part 25 (v4rho3sigma_7) CSE chunk 1027/1383 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpss_lxc_pol_part25_v4rho3sigma_7_chunk1027(t4724: f64, t8313: f64, t14169: f64, t3628: f64, t3630: f64, t10590: f64, t2175: f64, t4722: f64, t226: f64, t3610: f64, t3629: f64, t2169: f64, t4761: f64) -> (f64, f64, f64, f64, f64) {
    let t14220 = t8313 * t4724;
    let t14223 = t3628 * t14169 * t3630;
    let t14229 = t2175 * t10590 * t4722;
    let t14232 = t226 * t3610;
    let t14234 = t2175 * t3629 * t14232;
    let t14238 = t2169 * t4761;
    (t14220, t14223, t14229, t14234, t14238)
}
