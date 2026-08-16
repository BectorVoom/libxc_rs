//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 25 (v4rho3sigma_1) CSE chunk 959/1226 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part25_v4rho3sigma_1_chunk959<F: Float>(t3788: F, t835: F, t1336: F, t3795: F, t3799: F, t3853: F, t12353: F, t12356: F, t12358: F, t12361: F, t12366: F, t12371: F, t12375: F, t12379: F, t1341: F, t1363: F, t3733: F, t3778: F, t3858: F, t5246: F) -> F {
    let t12384 = t3788 * t835;
    let t12385 = t1336 * t12384;
    let t12386 = t12385 * t3795;
    let t12388 = t3799 * t3853;
    let t12390 = -F::cast_from(5.0_f64) / F::cast_from(128.0_f64) * t1363 * t12353 - F::cast_from(35.0_f64) / F::cast_from(384.0_f64) * t12356 + F::cast_from(7.0_f64) / F::cast_from(384.0_f64) * t12358 - t1363 * t12361 / F::cast_from(768.0_f64) - F::cast_from(119.0_f64) / F::cast_from(4608.0_f64) * t12366 - t5246 * t12371 / F::cast_from(128.0_f64) + F::cast_from(3.0_f64) / F::cast_from(16.0_f64) * t3733 * t12375 - t1341 * t12379 / F::cast_from(3072.0_f64) - t3778 * t3858 / F::cast_from(1024.0_f64) - F::cast_from(7.0_f64) / F::cast_from(768.0_f64) * t12386 + F::cast_from(7.0_f64) / F::cast_from(1536.0_f64) * t12388;
    t12390
}
