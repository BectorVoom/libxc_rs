//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1336/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1336<F: Float>(t1340: F, t21041: F, t109270: F, t9839: F, t109241: F, t21051: F, t109279: F, t6328: F, t1415: F, t20980: F, t109297: F, t6333: F, t32298: F, t33676: F, t21030: F, t33655: F) -> (F, F, F, F, F, F, F, F) {
    let t113448 = t1340 * t21041;
    let t113450 = t109270 * t9839;
    let t113452 = t109241 * t21051;
    let t113454 = t109279 * t6328;
    let t113456 = t1415 * t20980;
    let t113458 = t109297 * t6333;
    let t113460 = t33676 * t32298;
    let t113462 = t33655 * t21030;
    (t113448, t113450, t113452, t113454, t113456, t113458, t113460, t113462)
}
