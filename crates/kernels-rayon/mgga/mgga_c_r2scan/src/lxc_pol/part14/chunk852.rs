//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 852/1276 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk852(t2755: f64, t410: f64, t1823: f64, t963: f64, t2747: f64, t741: f64, t1827: f64, t1693: f64, t898: f64, t2483: f64, t697: f64, t5344: f64, t5346: f64, t5350: f64, t5354: f64, t5355: f64, t5360: f64) -> f64 {
    let t7720 = 8.0_f64 * t410 * t2755;
    let t7721 = t963 * t1823;
    let t7724 = 0.23392894490538584828e1_f64 * t2747 * t741;
    let t7725 = t963 * t1827;
    let t7727 = t898 * t1693;
    let t7730 = 0.1301229756036208781e0_f64 * t2483 * t697;
    let t7731 = 0.70178683471615754484e1_f64 * t5344 - 0.20779030926817756511e3_f64 * t5346 - t5350 - t5354 - 0.23392894490538584828e1_f64 * t5355 - t5360 + t7720 + 0.10254018858216406658e4_f64 * t7721 - t7724 - 0.23392894490538584828e1_f64 * t7725 - 0.1301229756036208781e0_f64 * t7727 + t7730;
    t7731
}
