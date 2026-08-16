//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 1044/1332 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk1044(t1198: f64, t1350: f64, t384: f64, t398: f64, t4552: f64, t997: f64, t12572: f64, t4488: f64, t1140: f64, t5171: f64, t1315: f64, t13787: f64) -> (f64, f64, f64, f64, f64) {
    let t18066 = t384 * t398 * t1198 * t1350;
    let t18072 = t997 * t4552;
    let t18079 = t12572 * t4488;
    let t18085 = t1140 * t5171;
    let t18087 = t13787 * t1315;
    (t18066, t18072, t18079, t18085, t18087)
}
