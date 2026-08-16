//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 830/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk830(t5189: f64, t7047: f64, t3426: f64, t496: f64, t501: f64, t5325: f64, t5339: f64, t5025: f64, t5028: f64, t5040: f64, t5066: f64, t5069: f64, t5073: f64, t5324: f64, t5333: f64, t5338: f64, t5344: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t8848 = 20.0_f64 * t5189;
    let t8849 = 0.21687162600603479684e-1_f64 * t7047;
    let t8850 = t496 * t3426;
    let t8851 = 4.0_f64 * t8850;
    let t8852 = t501 * t3426;
    let t8853 = 4.0_f64 * t8852;
    let t8854 = 0.24415263074675393405e-3_f64 * t5325;
    let t8855 = 0.5848223622634646207e0_f64 * t5339;
    let t8856 = t8848 + t5025 + t8849 + t5028 + t8851 - t8853 - t5324 + t5040 + t5066 - t5069 - t5073 + t8854 + t5333 - t5338 - t8855 - t5344;
    (t8848, t8849, t8850, t8851, t8852, t8853, t8854, t8855, t8856)
}
