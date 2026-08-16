//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1062/1420 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1062(t1017: f64, t13428: f64, t86: f64, t4557: f64, t3217: f64, t330: f64, t1646: f64, t3219: f64, t3210: f64, t3200: f64, t3203: f64, t3228: f64) -> (f64, f64, f64) {
    let t13430 = t86 * t1017 * t13428;
    let t13431 = t13430 * t4557;
    let t13433 = t3217 * t330;
    let t13435 = t13433 * t1646 * t3219;
    let t13436 = t3210 * t13435;
    let t13437 = t3200 * t13436;
    let t13440 = t3203 * t1646 * t3228;
    (t13431, t13437, t13440)
}
