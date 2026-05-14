//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 1112/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk1112<F: Float>(t33032: F, t33033: F, t7242: F, t3293: F, t5185: F, t9679: F, t1799: F, t3290: F, t6675: F, t5054: F, t6666: F, t32909: F, t9649: F, t18325: F, t9648: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t33034 = t33032 * t33033;
    let t33035 = t7242 * t33034;
    let t33040 = t5185 * t3293;
    let t33041 = t9679 * t33040;
    let t33042 = t1799 * t33041;
    let t33044 = t6675 * t3290;
    let t33045 = t9679 * t33044;
    let t33046 = t5054 * t33045;
    let t33048 = t6666 * t3290;
    let t33049 = t9679 * t33048;
    let t33050 = t1799 * t33049;
    let t33052 = t9649 * t32909;
    let t33056 = t9648 * t18325;
    (t33034, t33035, t33040, t33041, t33042, t33044, t33045, t33046, t33048, t33049, t33050, t33052, t33056)
}
