//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 805/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk805<F: Float>(t4828: F, t5033: F, t393: F, t1141: F, t1778: F, t1203: F, t1820: F, t3325: F, t3330: F, t359: F, t4772: F, t376: F) -> (F, F, F, F, F, F, F, F, F) {
    let t5034 = t4828 + t5033;
    let t5035 = t5034 * t393;
    let t5036 = t1778 * t1141;
    let t5037 = t5036 * t1203;
    let t5038 = t3325 * t1820;
    let t5039 = t1820 * t1203;
    let t5041 = F::cast_from(2.0_f64) * t3330 * t5039;
    let t5042 = t359 * t4772;
    let t5043 = t376 * t5042;
    (t5034, t5035, t5036, t5037, t5038, t5039, t5041, t5042, t5043)
}
