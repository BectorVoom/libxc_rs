//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 974/1419 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk974(t110: f64, t1263: f64, t1251: f64, t1258: f64, t1259: f64, t2888: f64, t992: f64, t1254: f64, t25: f64, t2887: f64, t3530: f64, t993: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t10989 = t110 * t1263;
    let t10990 = t1251 * t10989;
    let t10999 = t1258 * t1258;
    let t11000 = 1.0_f64 / t10999;
    let t11020 = t2888 * t1259;
    let t11061 = t110 * t992;
    let t11062 = t11061 * t1254;
    let t11063 = t1251 * t11062;
    let t11068 = t25 * t2887;
    let t11072 = t993 * t3530;
    (t10990, t11000, t11020, t11061, t11063, t11068, t11072)
}
