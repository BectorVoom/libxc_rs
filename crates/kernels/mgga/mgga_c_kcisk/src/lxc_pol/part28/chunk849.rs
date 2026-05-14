//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 849/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk849<F: Float>(t3193: F, t3206: F, t151: F, t816: F, t3199: F, t22: F, t955: F, t963: F, t15: F, t26: F, t146: F, t213: F, t3: F, t20: F, t2861: F, t952: F) -> (F, F, F, F, F, F, F, F) {
    let t12454 = t3193 * t3206;
    let t12459 = t816 * t151;
    let t12460 = t3199 * t12459;
    let t12462 = t22 * t963 * t955;
    let t12467 = 1.0 / t15 / t26 / 4.0;
    let t12468 = t12467 * t146;
    let t12469 = t3 * t213;
    let t12473 = t2861 * t952 * t20;
    (t12454, t12459, t12460, t12462, t12467, t12468, t12469, t12473)
}
