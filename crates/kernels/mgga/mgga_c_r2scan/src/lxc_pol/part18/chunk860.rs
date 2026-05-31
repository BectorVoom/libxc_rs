//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 860/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk860<F: Float>(t166: F, t8590: F, t3034: F, t607: F, t2483: F, t955: F, t2461: F, t898: F, t6030: F, t7126: F, t765: F, t7898: F, t7904: F, t8649: F, t8651: F, t8652: F) -> (F, F, F, F, F) {
    let t9063 = t8590 * t166;
    let t9066 = t3034 * t607;
    let t9069 = t2483 * t955;
    let t9072 = t898 * t2461;
    let t9075 = -F::cast_from(0.1143056e0_f64) * t7898 + F::cast_from(0.1350520664e0_f64) * t6030 - t8649 + t8651 + F::cast_from(0.675260332e-1_f64) * t765 * t9063 + F::cast_from(0.675260332e-1_f64) * t765 * t9066 + F::cast_from(0.1350520664e0_f64) * t765 * t9069 + F::cast_from(0.1350520664e0_f64) * t765 * t9072 - t7126 - t8652 - t7904;
    (t9063, t9066, t9069, t9072, t9075)
}
