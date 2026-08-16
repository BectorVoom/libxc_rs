//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1148/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1148(t19309: f64, t3227: f64, t1092: f64, t2861: f64, t6557: f64, t6498: f64, t10245: f64, t6496: f64, t1021: f64, t2825: f64, t6497: f64, t18443: f64, t313: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t19310 = t3227 * t19309;
    let t19311 = t1092 * t19310;
    let t19313 = t2861 * t6557;
    let t19315 = t2861 * t6498;
    let t19317 = t10245 * t6496;
    let t19318 = t1021 * t19317;
    let t19319 = t1092 * t19318;
    let t19321 = t2825 * t6497;
    let t19322 = t1092 * t19321;
    let t19324 = t313 * t18443;
    (t19311, t19313, t19315, t19319, t19322, t19324)
}
