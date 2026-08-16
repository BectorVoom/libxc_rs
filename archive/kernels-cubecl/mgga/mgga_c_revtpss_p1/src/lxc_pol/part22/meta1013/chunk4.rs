//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3483/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3483<F: Float>(t16158: F, t4834: F, t19791: F, t3127: F, t3172: F, t1042: F, t11977: F, t11994: F, t15839: F, t15847: F, t15850: F, t16149: F, t1675: F, t19649: F, t19878: F, t19930: F, t19940: F, t2858: F, t3188: F, t4831: F, t4837: F, t4875: F, t53353: F, t53926: F, t54137: F, t6302: F) -> F {
    let t65538 = t4834 * t16158;
    let t65553 = t3127 * t3172 * t19791;
    let t65563 = -F::cast_from(0.57165357490759649296e-3_f64) * t11994 * t19940 + F::cast_from(0.57165357490759649296e-3_f64) * t53353 + F::cast_from(0.3811023832717309953e-3_f64) * t65538 + F::cast_from(0.85748036236139473944e-3_f64) * t19878 * t15839 + F::cast_from(0.28582678745379824648e-3_f64) * t54137 * t1675 + F::cast_from(0.57165357490759649296e-3_f64) * t15850 * t4831 + F::cast_from(0.28582678745379824648e-3_f64) * t4834 * t15847 - F::cast_from(0.57165357490759649296e-3_f64) * t4837 * t1042 * t19649 * t2858 - F::cast_from(0.3811023832717309953e-3_f64) * t65553 + F::cast_from(0.57165357490759649296e-3_f64) * t19878 * t16149 + F::cast_from(0.30488190661738479624e-2_f64) * t53926 * t4875 + F::cast_from(0.17149607247227894789e-2_f64) * t3188 * t19930 - F::cast_from(0.22866142996303859718e-2_f64) * t11977 * t6302;
    t65563
}
