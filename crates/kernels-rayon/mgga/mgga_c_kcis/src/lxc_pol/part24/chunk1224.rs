//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1224/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1224(t5026: f64, t5096: f64, t1096: f64, t20194: f64, t1021: f64, t19911: f64, t19857: f64, t28029: f64, t26933: f64, t29056: f64, t29051: f64, t92544: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t99958 = t5026 * t5096;
    let t99960 = t1096 * t20194;
    let t99962 = t1021 * t19911;
    let t99964 = t28029 * t19857;
    let t99966 = t26933 * t29056;
    let t99968 = t92544 * t29051;
    (t99958, t99960, t99962, t99964, t99966, t99968)
}
