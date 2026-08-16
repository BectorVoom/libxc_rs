//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 638/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk638(t3435: f64, t3480: f64, t1142: f64, t20: f64, t2865: f64, t414: f64, t1242: f64, t1247: f64, t1241: f64, t68: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t3481 = t3435 + t3480;
    let t3482 = t1142 * t3481;
    let t3483 = t2865 * t20;
    let t3484 = t414 * t3483;
    let t3487 = t1242 * t1247;
    let t3489 = t1241 * t68;
    let t3490 = t414 * t3489;
    (t3481, t3482, t3483, t3484, t3487, t3490)
}
