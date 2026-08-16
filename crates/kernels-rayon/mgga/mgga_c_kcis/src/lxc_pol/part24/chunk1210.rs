//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1210/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1210(t27761: f64, t28302: f64, t27741: f64, t26657: f64, t29225: f64, t29228: f64, t29238: f64, t91769: f64, t91772: f64, t91773: f64, t91776: f64, t91777: f64, t91778: f64, t95270: f64, t95271: f64, t95272: f64, t95273: f64, t95274: f64, t95276: f64) -> (f64, f64, f64) {
    let t97607 = t27761 / 8.0_f64;
    let t97608 = t28302 / 8.0_f64;
    let t99798 = 4.0_f64 * t27741;
    let t99799 = t95270 - t91769 + t91772 + t29238 + t91773 + t95271 - t91776 - t95272 + t95273 + t91777 + t95274 + t99798 - t91778 + t26657 - t29225 - t95276 - t29228;
    (t97607, t97608, t99799)
}
