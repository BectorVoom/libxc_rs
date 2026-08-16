//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1307/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1307<F: Float>(t1498: F, t303: F, t7194: F, t27475: F, t7258: F, t20994: F, t4160: F, t98661: F, t102221: F, t18183: F, t18256: F, t20882: F, t20984: F, t2260: F, t27583: F, t28765: F, t6151: F, t8217: F, t94472: F, t94537: F, t94539: F, t94966: F, t99120: F, t99392: F, t99411: F) -> (F, F, F, F) {
    let t102381 = t303 * t7194 * t1498;
    let t102384 = t303 * t27475 * t7258;
    let t102398 = t4160 * t98661 * t20994;
    let t102400 = -F::cast_from(0.25794135802469135802e-3_f64) * t94472 + t99392 - F::cast_from(0.15445601851851851852e-3_f64) * t27583 * t6151 * t28765 * t20882 + F::cast_from(0.11607361111111111111e-2_f64) * t102381 - F::cast_from(0.17411041666666666666e-2_f64) * t102384 + t99411 + F::cast_from(0.18534722222222222222e-2_f64) * t18256 * t8217 * t2260 + F::cast_from(0.38691203703703703703e-3_f64) * t94537 - F::cast_from(0.25794135802469135802e-3_f64) * t94539 - F::cast_from(0.30945286961263020834e-5_f64) * t94966 * t102221 - F::cast_from(0.36039737654320987655e-3_f64) * t27583 * t18183 * t99120 * t20984 + F::cast_from(0.46429444444444444444e-2_f64) * t102398;
    (t102381, t102384, t102398, t102400)
}
