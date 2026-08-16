//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 830/1400 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk830(t1055: f64, t7624: f64, t1052: f64, t1635: f64, t1920: f64, t1956: f64, t388: f64, t4557: f64, t4660: f64, t6685: f64, t6687: f64, t6771: f64, t7554: f64, t7557: f64, t7562: f64, t7566: f64, t7569: f64, t7594: f64, t7600: f64) -> (f64, f64) {
    let t7625 = t1055 * t7624;
    let t7627 = t6685 + 0.27415567780803773942e-2_f64 * t6687 * t7554 - 0.82246703342411321825e-2_f64 * t6687 * t7557 + 0.82246703342411321825e-2_f64 * t1920 * t7562 - 0.82246703342411321825e-2_f64 * t6687 * t7566 + t7569 * t388 + t7594 * t388 - t6771 * t1635 - t4557 * t1956 - t4660 * t1956 + 2.0_f64 * t1052 * t7600 - t1052 * t7625;
    (t7625, t7627)
}
