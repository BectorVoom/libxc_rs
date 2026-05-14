//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1268/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1268<F: Float>(t32005: F, t3748: F, t32401: F, t9529: F, t42957: F, t79: F, t32422: F, t12261: F, t2737: F, t9543: F, t9518: F, t32446: F, t3973: F, t9536: F, t32473: F, t9535: F) -> (F, F, F, F, F, F, F, F, F) {
    let t109636 = t3748 * t32005;
    let t109643 = t9529 * t32401;
    let t109645 = t42957 * t79;
    let t109649 = t9529 * t32422;
    let t109652 = t2737 * t12261 * t9543;
    let t109654 = t12261 * t9518;
    let t109655 = t2737 * t109654;
    let t109662 = t9536 * t3973 * t32446;
    let t109664 = t32473 * t9535;
    (t109636, t109643, t109645, t109649, t109652, t109654, t109655, t109662, t109664)
}
