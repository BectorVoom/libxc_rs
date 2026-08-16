//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 681/1063 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk681(t1683: f64, t4761: f64, t5412: f64, t4730: f64, t827: f64, t10488: f64, t4726: f64, t26: f64, t10442: f64, t1659: f64, t1660: f64, t2877: f64) -> (f64, f64, f64, f64, f64) {
    let t10603 = t4761 * t1683;
    let t10604 = t10603 * t5412;
    let t10607 = t827 * t4730;
    let t10609 = t4726 * t10488;
    let t10610 = t26 * t10609;
    let t10612 = t1659 * t10442;
    let t10613 = t26 * t10612;
    let t10615 = t2877 * t1660;
    (t10604, t10607, t10610, t10613, t10615)
}
