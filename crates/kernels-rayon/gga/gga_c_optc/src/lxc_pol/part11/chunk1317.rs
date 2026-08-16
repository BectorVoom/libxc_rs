//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1317/1451 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1317(t10409: f64, t10478: f64, t16709: f64, t16774: f64, t16785: f64, t16872: f64, t16880: f64, t2476: f64, t24804: f64, t2493: f64, t2518: f64, t252: f64, t2520: f64, t2530: f64, t2537: f64, t31288: f64, t31301: f64, t31304: f64, t40949: f64, t41396: f64, t4869: f64, t4904: f64, t56677: f64, t56689: f64, t57352: f64, t57403: f64, t57416: f64, t57432: f64, t57447: f64, t57453: f64, t57501: f64, t57513: f64, t7753: f64, t7759: f64, t7799: f64, t7801: f64, t7813: f64, t810: f64, t818: f64, t837: f64) -> f64 {
    let t57517 = 0.82765347514623860983e4_f64 * t31288 * t16774 - 0.24829604254387158296e5_f64 * t24804 * t57352 * t7801 + 1.0_f64 * t810 * (t57403 + t57416 + t57432 + t57447) * t818 + 0.96494049533612093922e2_f64 * t2518 * t57453 * t2520 + 0.14035736153892489771e2_f64 * t10409 * t16880 - 0.1403573615389248977e2_f64 * t7813 * t56677 * t837 - 0.35089340384731224426e1_f64 * t2530 * t56689 * t837 + 0.51947267698127589897e2_f64 * t2537 * t56689 * t2476 + 24.0_f64 * t10478 * t16872 - 24.0_f64 * t7759 * t57352 * t818 - 6.0_f64 * t2493 * t57453 * t818 - 0.77195239626889675138e3_f64 * t31304 * t16709 + 0.11579285944033451271e4_f64 * t7799 * t57352 * t2520 - 0.70178680769462448852e1_f64 * t40949 * t4904 - 0.4155781415850207192e3_f64 * t31301 * t16785 + 0.6233672123775310788e3_f64 * t7753 * t56677 * t2476 - 12.0_f64 * t41396 * t4869 - 0.3109e-1_f64 * (t57501 + t57513) * t252;
    t57517
}
