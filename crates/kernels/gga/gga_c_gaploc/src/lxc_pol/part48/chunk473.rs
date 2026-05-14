//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 473/861 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk473<F: Float>(t2519: F, t3220: F, t3225: F, t716: F, t1890: F, t723: F, t2558: F, t7634: F, t9647: F, t1843: F, t7069: F, t7064: F, t2617: F, t948: F, t7803: F, t7802: F, t822: F) -> (F, F, F, F, F, F, F) {
    let t9674 = t3220 * t2519;
    let t9676 = t3225 * t716;
    let t9740 = t1890 * t723;
    let t9752 = t7634 * t2558;
    let t9754 = 0.64087718584518535698e-3 * t9647 * t9752;
    let t9760 = t1843 * t7069;
    let t9762 = 0.64087718584518535698e-3 * t7064 * t9760;
    let t9787 = t948 * t2617;
    let t9788 = t7803 * t9787;
    let t9789 = 0.38342925953920749676e0 * t9788;
    let t9796 = t822 * t7802;
    (t9674, t9676, t9740, t9754, t9762, t9789, t9796)
}
