//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3269/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3269<F: Float>(t10811: F, t18647: F, t18511: F, t40864: F, t10905: F, t18515: F, t10744: F, t18409: F, t808: F, t18414: F, t40521: F, t10900: F, t14468: F, t1548: F, t18393: F, t18444: F, t2430: F, t2724: F, t2730: F, t4362: F, t4364: F, t50968: F, t50974: F, t5984: F, t5988: F, t775: F, t800: F) -> F {
    let t62045 = t10811 * t18647;
    let t62056 = t40864 * t18511;
    let t62058 = t10905 * t18515;
    let t62069 = t10744 * t808 * t18409;
    let t62072 = t40521 * t808 * t18414;
    let t62074 = F::cast_from(0.20007875121765877254e-2_f64) * t50968 + F::cast_from(0.12862205435420921092e-2_f64) * t4362 * t4364 * t18444 * t2724 - F::cast_from(0.16006300097412701803e-1_f64) * t62045 + F::cast_from(0.80031500487063509016e-1_f64) * t50974 + t2730 * t800 * t18393 * t775 / F::cast_from(8.0_f64) + t2730 * t800 * t5984 * t2430 / F::cast_from(16.0_f64) + F::cast_from(7.0_f64) / F::cast_from(6.0_f64) * t62056 - F::cast_from(7.0_f64) / F::cast_from(12.0_f64) * t62058 - t10900 * t800 * t5988 * t2430 / F::cast_from(4.0_f64) + t2730 * t800 * t1548 * t14468 / F::cast_from(8.0_f64) + F::cast_from(0.25410001404642664112e-5_f64) * t62069 - F::cast_from(0.50820002809285328225e-5_f64) * t62072;
    t62074
}
