//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 810/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk810<F: Float>(t1900: F, t227: F, t5737: F, t5802: F, t1954: F, t1972: F, t721: F, t730: F, t5519: F, t5522: F, t5525: F, t5539: F) -> (F, F, F, F, F, F, F) {
    let t5804 = F::new(1.0) / t1900 / t227;
    let t5805 = t5737 * t5804;
    let t5807 = F::cast_from(0.51726012919273400301e3_f64) * t5802 * t5805;
    let t5809 = t1954 * t721 * t1972;
    let t5811 = F::cast_from(0.35089341735807877242e1_f64) * t730 * t5809;
    let t5812 = F::cast_from(0.53272592592592592592e-1_f64) * t5519;
    let t5816 = -t5812 + F::cast_from(0.68493333333333333332e-1_f64) * t5522 - F::cast_from(0.51369999999999999999e-1_f64) * t5525 + F::new(0.5137e-1) * t5539;
    (t5804, t5805, t5807, t5809, t5811, t5812, t5816)
}
