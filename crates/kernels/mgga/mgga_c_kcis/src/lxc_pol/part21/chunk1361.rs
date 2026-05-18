//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1361/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1361<F: Float>(t15573: F, t28178: F, t7788: F, t28183: F, t7772: F, t27014: F, t27028: F, t28132: F, t28137: F, t46849: F, t47050: F, t5329: F, t92861: F, t92872: F, t92890: F, t95949: F, t95952: F) -> F {
    let t97102 = F::new(0.46336805555555555556e-3) * t7788 * t15573 * t28178;
    let t97103 = t15573 * t28183;
    let t97105 = F::new(0.23168402777777777778e-3) * t7788 * t97103;
    let t97106 = t7772 * t97103;
    let t97125 = -t97102 - t97105 - F::new(0.30918233506944444444e-4) * t97106 - F::new(0.69505208333333333334e-3) * t27014 * t28132 - F::new(0.13901041666666666667e-2) * t27014 * t28137 - F::new(0.69505208333333333334e-3) * t7788 * t5329 * t27028 * t47050 - F::new(0.23214722222222222222e-2) * t95949 - F::new(0.11607361111111111111e-2) * t95952 - F::new(0.23168402777777777778e-3) * t92861 + F::new(0.10317654320987654321e-2) * t92872 - F::new(0.3861400462962962963e-4) * t92890 - F::new(0.13901041666666666667e-2) * t7788 * t5329 * t27028 * t46849;
    t97125
}
