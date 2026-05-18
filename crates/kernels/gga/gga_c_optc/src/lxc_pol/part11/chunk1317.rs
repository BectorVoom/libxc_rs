//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 1317/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk1317<F: Float>(t10409: F, t10478: F, t16709: F, t16774: F, t16785: F, t16872: F, t16880: F, t2476: F, t24804: F, t2493: F, t2518: F, t252: F, t2520: F, t2530: F, t2537: F, t31288: F, t31301: F, t31304: F, t40949: F, t41396: F, t4869: F, t4904: F, t56677: F, t56689: F, t57352: F, t57403: F, t57416: F, t57432: F, t57447: F, t57453: F, t57501: F, t57513: F, t7753: F, t7759: F, t7799: F, t7801: F, t7813: F, t810: F, t818: F, t837: F) -> F {
    let t57517 = F::new(0.82765347514623860983e4) * t31288 * t16774 - F::new(0.24829604254387158296e5) * t24804 * t57352 * t7801 + F::new(1.0) * t810 * (t57403 + t57416 + t57432 + t57447) * t818 + F::new(0.96494049533612093922e2) * t2518 * t57453 * t2520 + F::new(0.14035736153892489771e2) * t10409 * t16880 - F::new(0.1403573615389248977e2) * t7813 * t56677 * t837 - F::new(0.35089340384731224426e1) * t2530 * t56689 * t837 + F::new(0.51947267698127589897e2) * t2537 * t56689 * t2476 + F::new(24.0) * t10478 * t16872 - F::new(24.0) * t7759 * t57352 * t818 - F::new(6.0) * t2493 * t57453 * t818 - F::new(0.77195239626889675138e3) * t31304 * t16709 + F::new(0.11579285944033451271e4) * t7799 * t57352 * t2520 - F::new(0.70178680769462448852e1) * t40949 * t4904 - F::new(0.4155781415850207192e3) * t31301 * t16785 + F::new(0.6233672123775310788e3) * t7753 * t56677 * t2476 - F::new(12.0) * t41396 * t4869 - F::new(0.3109e-1) * (t57501 + t57513) * t252;
    t57517
}
