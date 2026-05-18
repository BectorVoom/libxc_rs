//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1239/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1239<F: Float>(t27357: F, t5426: F, t98233: F, t12185: F, t1307: F, t1650: F, t27359: F, t27369: F, t27459: F, t28439: F, t3984: F, t4001: F, t7908: F, t7909: F, t94227: F, t94287: F, t94289: F, t94626: F, t98155: F, t98205: F, t98220: F, t98226: F, t98230: F) -> (F, F) {
    let t98235 = t98233 * t5426 * t27357;
    let t98238 = F::new(0.46336805555555555556e-3) * t7908 * t3984 * t98205 * t1307 - F::new(0.16489724537037037037e-3) * t98155 * t27359 - F::new(0.46336805555555555556e-3) * t7908 * t12185 * t7909 * t1650 * t4001 + F::new(0.46336805555555555556e-3) * t27459 * t28439 - F::new(0.46336805555555555556e-3) * t7908 * t98220 - F::new(0.61836467013888888888e-4) * t27369 * t98220 + t98226 + F::new(0.10297067901234567901e-3) * t94287 - F::new(0.15445601851851851852e-3) * t94289 - F::new(0.46336805555555555556e-3) * t94626 * t98230 + F::new(0.82448622685185185186e-4) * t94227 * t98235;
    (t98235, t98238)
}
