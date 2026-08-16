//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1175/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1175(t283: f64, t3177: f64, t2193: f64, t2196: f64, t44682: f64, t1086: f64, t3225: f64, t3245: f64, t7727: f64, t7735: f64, t26972: f64, t7780: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t92917 = t3177 * t283;
    let t92964 = 0.12871334876543209877e-3_f64 * t2193 * t44682 * t2196;
    let t92972 = t1086 * t3225;
    let t92993 = t3245 * t7727;
    let t92997 = t3245 * t7735;
    let t93016 = t7780 * t26972;
    (t92917, t92964, t92972, t92993, t92997, t93016)
}
