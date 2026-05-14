//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1050/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1050<F: Float>(t1957: F, t9094: F, t5218: F, t2587: F, t7410: F, t2568: F, t7401: F, t2560: F, t7330: F, t7337: F, t7424: F, t17975: F, t5061: F, t7431: F, t17939: F, t23733: F) -> (F, F, F, F, F, F, F, F) {
    let t24098 = t9094 * t1957;
    let t24100 = 2.0 * t5218 * t24098;
    let t24101 = t7410 * t2587;
    let t24103 = t7401 * t2568;
    let t24105 = t2560 * t7330;
    let t24107 = t7337 * t7424;
    let t24109 = t5061 * t17975;
    let t24110 = t24109 * t7431;
    let t24112 = t17939 * t23733;
    (t24098, t24100, t24101, t24103, t24105, t24107, t24110, t24112)
}
