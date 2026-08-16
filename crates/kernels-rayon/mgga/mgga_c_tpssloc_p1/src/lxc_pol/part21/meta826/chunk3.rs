//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2915/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2915(t10756: f64, t10765: f64, t10828: f64, t13716: f64, t14271: f64, t14276: f64, t14425: f64, t14429: f64, t14432: f64, t14436: f64, t17492: f64, t17499: f64, t17535: f64, t2905: f64, t2906: f64, t2924: f64, t2930: f64, t42111: f64, t42113: f64, t4416: f64, t4438: f64, t4475: f64, t48789: f64, t49427: f64, t49430: f64, t5774: f64, t5791: f64, t60722: f64, t60741: f64, t60744: f64, t60748: f64, t60750: f64, t60752: f64) -> f64 {
    let t60763 = -0.11696447245269292414e1_f64 * t2905 * t5791 * t2924 - 0.10389515463408878255e3_f64 * t10828 * t17492 * t2906 + 0.17315859105681463759e2_f64 * t2930 * t17492 * t2924 + 0.10254018858216406658e4_f64 * t10756 * t60722 * t2906 + 0.34631718211362927518e2_f64 * t2930 * t4475 * t13716 + 0.10254018858216406658e4_f64 * t10756 * t17499 * t2924 + 0.91082604192152556044e5_f64 * t42111 * t5774 * t42113 * t2906 - 4.0_f64 * t14276 * t14429 - 0.38596750796862084161e3_f64 * t49430 * t14432 - t60741 + t60744 - t60748 - t60750 - t60752 - 8.0_f64 * t49427 * t4416 + 0.12865583598954028054e3_f64 * t48789 * t4438 - 8.0_f64 * t14276 * t14425 + 0.12865583598954028054e3_f64 * t14271 * t14436 + 12.0_f64 * t10765 * t17535;
    t60763
}
