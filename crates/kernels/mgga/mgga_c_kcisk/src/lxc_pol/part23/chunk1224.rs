//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1224/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1224<F: Float>(t1520: F, t9848: F, t4170: F, t394: F, t6387: F, t9492: F, t32278: F, t6328: F, t19861: F, t500: F, t488: F, t6309: F, t32277: F, t3784: F) -> (F, F, F, F, F, F, F, F) {
    let t33640 = t9848 * t1520;
    let t33642 = 2.0 * t4170 * t33640;
    let t33643 = t6387 * t394;
    let t33644 = t33643 * t9492;
    let t33646 = t32278 * t6328;
    let t33648 = t19861 * t500;
    let t33650 = t6309 * t488;
    let t33652 = t3784 * t32277;
    (t33640, t33642, t33643, t33644, t33646, t33648, t33650, t33652)
}
