//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 811/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk811<F: Float>(t1224: F, t1697: F, t28385: F, t28389: F, t11105: F, t17382: F, t23460: F, t23472: F, t23481: F, t29082: F, t29085: F, t29088: F, t29091: F, t2430: F, t8746: F, t1746: F, t4954: F) -> (F, F, F, F, F) {
    let t29094 = t1224 * t1697 * t28385;
    let t29097 = t1224 * t1697 * t28389;
    let t29099 = -t11105 - 0.23744444444444444444e-1 * t17382 + 0.11872222222222222222e-1 * t23460 - 0.35616666666666666666e-1 * t23472 + 0.17808333333333333333e-1 * t23481 - 0.19787037037037037037e-1 * t29082 + 0.71233333333333333332e-1 * t29085 - 0.35616666666666666666e-1 * t29088 - 0.10685e0 * t29091 + 0.10685e0 * t29094 - 0.17808333333333333333e-1 * t29097;
    let t29102 = t8746 * t2430;
    let t29104 = t4954 * t29102 * t1746;
    (t29094, t29097, t29099, t29102, t29104)
}
