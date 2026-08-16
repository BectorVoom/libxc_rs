//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1037/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1037(t1415: f64, t6823: f64, t1559: f64, t197: f64, t1563: f64, t202: f64, t2486: f64, t4786: f64, t1428: f64, t4360: f64, t15478: f64, t585: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t18482 = t1415 * t6823;
    let t18535 = t1559 * t197;
    let t18540 = 1.0_f64 / t1563 / t202;
    let t18676 = t4786 * t2486;
    let t18736 = t4360 * t1428;
    let t18821 = t585 * t15478;
    (t18482, t18535, t18540, t18676, t18736, t18821)
}
