//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3489/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3489<F: Float>(t15772: F, t4834: F, t1042: F, t1045: F, t15830: F, t15850: F, t18281: F, t19622: F, t19675: F, t19682: F, t2858: F, t3059: F, t3075: F, t3106: F, t3117: F, t3127: F, t42121: F, t42124: F, t42141: F, t43291: F, t43297: F, t4803: F, t4872: F, t53389: F, t5825: F, t6271: F, t999: F) -> F {
    let t65689 = t4834 * t15772;
    let t65693 = -F::cast_from(0.28582678745379824648e-3_f64) * t3127 * t1042 * t4872 * t18281 * t999 - F::cast_from(0.14291339372689912324e-3_f64) * t3127 * t1042 * t4872 * t5825 * t3075 - t42121 - F::cast_from(0.47637797908966374413e-4_f64) * t42124 - F::cast_from(0.16090989515917530913e-2_f64) * t42141 - F::cast_from(0.25724410870841842183e-2_f64) * t43291 * t3117 * t6271 * t1045 * t3059 - F::cast_from(0.91464571985215438873e-2_f64) * t43297 * t19622 + F::cast_from(0.19055119163586549765e-3_f64) * t53389 + F::cast_from(0.28582678745379824648e-3_f64) * t3127 * t1042 * t19675 * t2858 + F::cast_from(0.30488190661738479624e-2_f64) * t3106 * t19682 + F::cast_from(0.60976381323476959249e-2_f64) * t15830 * t4803 - F::cast_from(0.76220476654346199061e-3_f64) * t65689 - F::cast_from(0.11433071498151929859e-2_f64) * t15850 * t4803;
    t65693
}
