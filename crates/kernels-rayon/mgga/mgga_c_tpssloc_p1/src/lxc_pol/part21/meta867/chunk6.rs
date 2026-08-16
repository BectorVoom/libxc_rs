//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3170/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3170(t18392: f64, t3490: f64, t1227: f64, t18241: f64, t248: f64, t3521: f64, t19040: f64, t15734: f64, t5024: f64, t11818: f64, t3515: f64, t6230: f64) -> (f64, f64, f64, f64, f64) {
    let t65613 = t3490 * t18392;
    let t65617 = t1227 * t248 * t3521 * t18241;
    let t65619 = t3490 * t19040;
    let t65628 = t5024 * t15734;
    let t65632 = t3515 * t248 * t11818 * t6230;
    (t65613, t65617, t65619, t65628, t65632)
}
