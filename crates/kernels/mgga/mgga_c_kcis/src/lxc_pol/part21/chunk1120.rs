//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1120/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1120<F: Float>(t27120: F, t27133: F, t2205: F, t3670: F, t11220: F, t11223: F, t11230: F, t1282: F, t1291: F, t26877: F, t26885: F, t26951: F, t27095: F, t27100: F, t27105: F, t3664: F, t3669: F, t437: F, t7812: F, t7823: F) -> (F, F, F) {
    let t27134 = t27120 + t27133;
    let t27136 = t2205 * t3670;
    let t27139 = -t11220 * t2205 + F::cast_from(4.0_f64) * t11223 * t7812 - F::cast_from(6.0_f64) * t11230 * t27136 - t1282 * t27134 - F::cast_from(2.0_f64) * t1291 * t27100 + t27095 * t437 + F::cast_from(4.0_f64) * t27105 * t3669 - F::cast_from(2.0_f64) * t3664 * t7823 - t26877 - t26885 + t26951;
    (t27134, t27136, t27139)
}
