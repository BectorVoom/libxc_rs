//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2541/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2541(t21780: f64, t3287: f64, t1102: f64, t3270: f64, t21785: f64, t43880: f64, t18754: f64, t4756: f64, t14808: f64, t5999: f64, t18730: f64, t4748: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t71445 = t3287 * t21780;
    let t71446 = t71445 * t1102;
    let t71448 = t3270 * t21780;
    let t71449 = t71448 * t1102;
    let t71452 = t43880 * t21785 * t1102;
    let t71454 = t18754 * t4756;
    let t71456 = t14808 * t5999;
    let t71458 = t4748 * t18730;
    (t71446, t71449, t71452, t71454, t71456, t71458)
}
