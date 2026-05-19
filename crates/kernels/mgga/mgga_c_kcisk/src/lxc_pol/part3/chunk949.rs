//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 949/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk949<F: Float>(t12952: F, t1375: F, t11525: F, t435: F, t437: F, t11529: F, t447: F, t445: F, t3845: F, t429: F, t431: F, t12868: F, t1398: F) -> (F, F, F, F, F) {
    let t14047 = t1375 * t12952;
    let t14056 = F::cast_from(0.77488888888888888888e-2_f64) * t435 * t11525 * t437;
    let t14057 = t11529 * t447;
    let t14059 = F::cast_from(0.72818958333333333333e-4_f64) * t445 * t14057;
    let t14062 = F::cast_from(0.27323333333333333333e-1_f64) * t429 * t3845 * t431;
    let t14063 = t1398 * t12868;
    (t14047, t14056, t14059, t14062, t14063)
}
