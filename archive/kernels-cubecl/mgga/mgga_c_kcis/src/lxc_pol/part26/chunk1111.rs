//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1111/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1111<F: Float>(t1014: F, t8171: F, t1380: F, t1650: F, t7909: F, t3984: F, t2237: F, t27483: F, t27486: F, t28501: F, t28506: F, t28508: F, t28511: F, t28514: F, t28517: F, t28520: F, t28522: F, t28526: F, t28529: F, t28532: F, t28535: F, t28544: F, t7895: F, t7901: F, t7908: F, t7916: F, t8151: F, t8159: F) -> (F, F, F, F, F) {
    let t28547 = t1014 * t8171;
    let t28549 = t1650 * t1380;
    let t28550 = t7909 * t28549;
    let t28551 = t3984 * t28550;
    let t28554 = F::cast_from(0.16581944444444444444e-2_f64) * t28501 + F::cast_from(0.49745833333333333332e-2_f64) * t28506 - F::cast_from(0.44218518518518518517e-2_f64) * t28508 + F::cast_from(0.11054629629629629629e-2_f64) * t28511 - F::cast_from(0.33163888888888888888e-2_f64) * t28514 + F::cast_from(0.27636574074074074073e-2_f64) * t28517 - F::cast_from(0.16581944444444444444e-2_f64) * t28520 - t27483 + t27486 + F::cast_from(0.23168402777777777778e-3_f64) * t28522 - F::cast_from(0.24872916666666666666e-2_f64) * t28526 + F::cast_from(0.16581944444444444444e-2_f64) * t28529 - F::cast_from(0.24872916666666666666e-2_f64) * t28532 + F::cast_from(0.69505208333333333333e-3_f64) * t2237 * t28535 + F::cast_from(0.69505208333333333333e-3_f64) * t7895 * t8159 - F::cast_from(0.18534722222222222222e-2_f64) * t8151 * t7916 - F::cast_from(0.18534722222222222222e-2_f64) * t8151 * t7901 - F::cast_from(0.24734586805555555555e-3_f64) * t28544 * t7901 - F::cast_from(0.16581944444444444444e-2_f64) * t28547 + F::cast_from(0.23168402777777777778e-3_f64) * t7908 * t28551;
    (t28547, t28549, t28550, t28551, t28554)
}
