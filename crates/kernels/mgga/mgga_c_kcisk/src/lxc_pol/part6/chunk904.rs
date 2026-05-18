//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 904/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk904<F: Float>(t1224: F, t28373: F, t4840: F, t28377: F, t1697: F, t28381: F, t28385: F, t28389: F, t11105: F, t17382: F, t23460: F, t23472: F, t23481: F, t29082: F) -> (F, F, F, F, F, F) {
    let t29085 = t1224 * t4840 * t28373;
    let t29088 = t1224 * t4840 * t28377;
    let t29091 = t1224 * t1697 * t28381;
    let t29094 = t1224 * t1697 * t28385;
    let t29097 = t1224 * t1697 * t28389;
    let t29099 = -t11105 - F::new(0.23744444444444444444e-1) * t17382 + F::new(0.11872222222222222222e-1) * t23460 - F::new(0.35616666666666666666e-1) * t23472 + F::new(0.17808333333333333333e-1) * t23481 - F::new(0.19787037037037037037e-1) * t29082 + F::new(0.71233333333333333332e-1) * t29085 - F::new(0.35616666666666666666e-1) * t29088 - F::new(0.10685e0) * t29091 + F::new(0.10685e0) * t29094 - F::new(0.17808333333333333333e-1) * t29097;
    (t29085, t29088, t29091, t29094, t29097, t29099)
}
