//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 981/1221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk981<F: Float>(t26725: F, t26776: F, t26822: F, t26864: F, t393: F, t1141: F, t7738: F, t1203: F, t2183: F, t3329: F, t3331: F, t3481: F, t7740: F, t10488: F, t2189: F, t10491: F, t7743: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26866 = t26725 + t26776 + t26822 + t26864;
    let t26867 = t26866 * t393;
    let t26868 = t7738 * t1141;
    let t26870 = 2.0 * t26868 * t1203;
    let t26871 = t2183 * t3329;
    let t26873 = 2.0 * t26871 * t3331;
    let t26874 = t7740 * t3481;
    let t26875 = t10488 * t2189;
    let t26877 = 4.0 * t10491 * t7743;
    (t26866, t26867, t26868, t26870, t26871, t26873, t26874, t26875, t26877)
}
