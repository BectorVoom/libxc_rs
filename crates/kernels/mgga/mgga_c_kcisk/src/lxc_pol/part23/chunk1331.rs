//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1331/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1331<F: Float>(t113375: F, t9498: F, t4208: F, t487: F, t20635: F, t32290: F, t33676: F, t12817: F, t2279: F, t21077: F, t32260: F, t32287: F, t6318: F, t20961: F, t9497: F, t21283: F, t9491: F) -> (F, F, F, F, F, F, F, F) {
    let t113376 = t113375 * t9498;
    let t113378 = t4208 * t487;
    let t113379 = t113378 * t20635;
    let t113381 = t33676 * t32290;
    let t113383 = t12817 * t2279;
    let t113385 = t32260 * t21077;
    let t113387 = t32287 * t6318;
    let t113389 = t9497 * t20961;
    let t113391 = t9491 * t21283;
    (t113376, t113379, t113381, t113383, t113385, t113387, t113389, t113391)
}
