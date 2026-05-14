//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1288/1296 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1288<F: Float>(t1853: F, t3721: F, t12281: F, t123: F, t12312: F, t12318: F, t169: F, t1841: F, t1897: F, t1908: F, t1939: F, t270: F, t29471: F, t29473: F, t299: F, t32560: F, t32585: F, t32588: F, t32591: F, t32594: F, t3727: F, t39121: F, t39181: F, t5227: F, t5524: F, t650: F, t706: F, t734: F, t779: F) -> (F,) {
    let t39454 = t3721 * t1853;
    let t39464 = t32560 - 0.8545029144602471425e-3 * t5524 * t12318 - 0.20508069947045931424e-1 * t650 * t12281 - 0.34180116578409885707e-2 * t1908 * t3727 - 0.20508069947045931424e-1 * t1939 * t3727 + 0.76905262301422242837e-2 * t270 * t706 * t39181 * t169 * t299 - 0.15381052460284448567e-1 * t1897 * t779 * t39454 - t29471 + t29473 - 0.17090058289204942853e-2 * t5227 * t12312 - 0.17090058289204942853e-2 * t1841 * t39121 * t123 * t734 - t32585 + t32588 + t32591 - t32594;
    (t39464,)
}
