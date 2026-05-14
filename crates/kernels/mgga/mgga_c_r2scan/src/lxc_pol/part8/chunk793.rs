//! MGGA_C_R2SCAN lxc pol — lxc_pol part 8 (v4rho4_3) CSE chunk 793/1467 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part8_v4rho4_3_chunk793<F: Float>(t171: F, t584: F, t5861: F, t61: F, t718: F, t226: F, t5456: F, t160: F, t35: F, t164: F, t1774: F, t604: F, t1780: F, t601: F, t2099: F, t161: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t5864 = 0.571528e-1 * t584 * t171 * t5861;
    let t5865 = t61 * t718;
    let t5866 = t226 * t5456;
    let t5868 = 0.10526802520742363173e2 * t5865 * t5866;
    let t5869 = t160 * t35;
    let t5871 = 1320.0 * t5869 * t164;
    let t5872 = t1774 * t604;
    let t5873 = 4752.0 * t5872;
    let t5874 = t601 * t1780;
    let t5875 = 5616.0 * t5874;
    let t5876 = 1.0 / t2099;
    let t5878 = 2184.0 * t161 * t5876;
    (t5864, t5865, t5866, t5868, t5869, t5871, t5872, t5873, t5874, t5875, t5876, t5878)
}
