//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3522/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3522<F: Float>(t11250: F, t16027: F, t16103: F, t16104: F, t16228: F, t19722: F, t19986: F, t3095: F, t43069: F, t53402: F, t54490: F, t54497: F, t54500: F, t54521: F, t54533: F, t54801: F, t54811: F, t55046: F, t55141: F, t66187: F, t66686: F, t66689: F, t66702: F, t66712: F, t66714: F) -> F {
    let t66716 = F::cast_from(0.28582678745379824648e-3_f64) * t54490 - F::cast_from(0.57165357490759649296e-3_f64) * t54497 + t66686 / F::new(432.0) + F::cast_from(0.11433071498151929859e-2_f64) * t43069 * t66689 * t16103 + F::cast_from(0.57165357490759649296e-3_f64) * t54521 - F::cast_from(0.57165357490759649296e-3_f64) * t55141 * t16104 + F::cast_from(0.30488190661738479624e-2_f64) * t53402 * t19986 - F::cast_from(0.17149607247227894789e-2_f64) * t54801 * t66187 * t11250 * t16228 + F::cast_from(0.28582678745379824648e-3_f64) * t54811 * t66187 * t66702 * t3095 + F::cast_from(0.57165357490759649296e-3_f64) * t54533 - F::cast_from(0.22866142996303859718e-2_f64) * t55046 * t19722 + F::cast_from(0.85748036236139473944e-3_f64) * t54500 * t16027 - F::cast_from(0.57165357490759649296e-3_f64) * t66712 - t66714 / F::new(162.0);
    t66716
}
