//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2585/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2585(t52367: f64, t11549: f64, t1174: f64, t44586: f64, t44589: f64, t44592: f64, t44595: f64, t44602: f64, t44628: f64, t44631: f64, t44635: f64, t44638: f64, t44641: f64, t457: f64, t460: f64, t4889: f64, t52327: f64, t52345: f64, t52355: f64, t52357: f64, t52362: f64, t52364: f64, t974: f64) -> f64 {
    let t52368 = 0.18518518518518518518e-3_f64 * t52367;
    let t52374 = -0.37037037037037037036e-3_f64 * t44586 + 0.27777777777777777777e-3_f64 * t44589 - 0.55555555555555555554e-3_f64 * t44592 + 0.37037037037037037036e-3_f64 * t44595 + 0.55555555555555555554e-3_f64 * t44602 - 0.83333333333333333332e-3_f64 * t1174 * t974 * t457 * (t52327 + t52345) * t460 + t52355 - 0.27777777777777777777e-3_f64 * t52357 + 0.23045267489711934156e-2_f64 * t4889 * t11549 - 0.83333333333333333331e-3_f64 * t52362 - 0.98765432098765432097e-3_f64 * t52364 + t52368 - 0.83333333333333333331e-3_f64 * t44628 - 0.9259259259259259259e-4_f64 * t44631 - 0.3086419753086419753e-3_f64 * t44635 + 0.18518518518518518518e-3_f64 * t44638 + 0.37037037037037037036e-3_f64 * t44641;
    t52374
}
