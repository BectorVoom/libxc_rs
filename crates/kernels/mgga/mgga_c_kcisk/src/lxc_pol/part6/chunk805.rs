//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 805/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk805<F: Float>(t1869: F, t28952: F, t23969: F, t23976: F, t23978: F, t28790: F, t28794: F, t28797: F, t28803: F, t28807: F, t28811: F, t28815: F, t28818: F, t652: F, t743: F, t719: F, sigma2: F) -> (F, F, F, F, F) {
    let t28953 = t1869 * t28952;
    let t28955 = 0.99491666666666666664e-2 * t23969 + 0.99491666666666666664e-2 * t28790 + 0.2653111111111111111e-1 * t28794 + 0.2653111111111111111e-1 * t28797 + 0.2653111111111111111e-1 * t23976 - 0.16581944444444444444e-2 * t28803 - 0.13265555555555555555e-1 * t28807 - 0.22109259259259259258e-1 * t28811 - 0.16581944444444444444e-1 * t28815 - 0.99491666666666666664e-2 * t28818 + 0.66327777777777777776e-2 * t23978 - 0.24872916666666666666e-2 * t28953;
    let t28957 = 1.0 / t652 / t743;
    let t28958 = sigma2 * t28957;
    let t28959 = t28958 * t719;
    (t28953, t28955, t28957, t28958, t28959)
}
