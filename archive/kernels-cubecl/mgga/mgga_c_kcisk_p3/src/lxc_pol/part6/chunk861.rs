//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 861/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk861<F: Float>(t28377: F, t4726: F, t26: F, t6777: F, t8522: F, t2372: F, t8504: F, t10663: F, t10671: F, t10621: F, t28369: F, t1659: F, t28389: F) -> (F, F, F, F, F, F, F) {
    let t28409 = t4726 * t28377;
    let t28410 = t26 * t28409;
    let t28412 = t6777 * t8522;
    let t28414 = t8504 * t2372;
    let t28415 = t10663 * t28414;
    let t28417 = t10671 * t28414;
    let t28419 = t10621 * t28369;
    let t28420 = t26 * t28419;
    let t28422 = t1659 * t28389;
    (t28410, t28412, t28414, t28415, t28417, t28420, t28422)
}
