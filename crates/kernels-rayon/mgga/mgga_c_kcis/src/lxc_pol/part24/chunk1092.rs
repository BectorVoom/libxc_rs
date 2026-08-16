//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1092/1322 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1092(t1250: f64, t19674: f64, t6625: f64, t7718: f64, t1020: f64, t19164: f64, t7704: f64, t2894: f64, t356: f64, t6556: f64, t303: f64, t6544: f64, t7691: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let t28932 = t19674 * t1250;
    let t28935 = t7718 * t6625;
    let t28936 = t1020 * t28935;
    let t28938 = t7704 * t19164;
    let t28939 = t2894 * t28938;
    let t28944 = t356 * t6556;
    let t28945 = t303 * t28944;
    let t28947 = t7691 * t6544;
    (t28932, t28935, t28936, t28938, t28939, t28944, t28945, t28947)
}
