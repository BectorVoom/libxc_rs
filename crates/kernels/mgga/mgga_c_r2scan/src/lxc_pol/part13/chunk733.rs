//! MGGA_C_R2SCAN lxc pol — lxc_pol part 13 (v4rho3sigma_3) CSE chunk 733/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part13_v4rho3sigma_3_chunk733<F: Float>(t5865: F, t5866: F, t160: F, t35: F, t164: F, t1774: F, t604: F, t1780: F, t601: F, t2099: F, t161: F, t2036: F, t406: F) -> (F, F, F, F, F, F) {
    let t5868 = F::cast_from(0.10526802520742363173e2_f64) * t5865 * t5866;
    let t5869 = t160 * t35;
    let t5871 = F::cast_from(1320.0_f64) * t5869 * t164;
    let t5872 = t1774 * t604;
    let t5874 = t601 * t1780;
    let t5876 = F::cast_from(1.0_f64) / t2099;
    let t5878 = F::cast_from(2184.0_f64) * t161 * t5876;
    let t5883 = t406 * t2036;
    (t5868, t5871, t5872, t5874, t5878, t5883)
}
