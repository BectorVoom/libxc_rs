//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 545/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk545<F: Float>(t1328: F, t8059: F, t2173: F, t3924: F, t1220: F, t2174: F, t3807: F, t3930: F, t412: F, t5880: F, t5972: F, t5979: F, t6221: F, t7828: F, t7834: F, t7837: F, t7840: F, t7909: F) -> (F, F, F, F) {
    let t8060 = t8059 * t1328;
    let t8063 = t2173 * t2173;
    let t8064 = t8063 * t3924;
    let t8071 = t7828 * t412 + 0.16581944444444444444e-2 * t7834 - 0.49745833333333333332e-2 * t7837 + 0.33163888888888888888e-2 * t7840 - 0.24872916666666666666e-2 * t7909 - t3807 + 0.33163888888888888888e-2 * t5880 - 0.193e0 * t1220 * t8060 + 0.74498e-1 * t3930 * t8064 - 0.33163888888888888888e-2 * t5972 + 0.22109259259259259258e-2 * t5979 - 0.386e0 * t6221 * t2174;
    (t8060, t8063, t8064, t8071)
}
