//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1326/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1326<F: Float>(t1014: F, t29383: F, t102328: F, t102698: F, t102701: F, t102706: F, t102709: F, t102712: F, t102715: F, t102723: F, t18128: F, t27583: F, t27584: F, t28701: F, t77753: F, t7978: F, t99248: F, t99646: F) -> (F, F) {
    let t102725 = t1014 * t29383;
    let t102727 = F::cast_from(0.15476481481481481481e-2_f64) * t102698 - F::cast_from(0.30952962962962962962e-2_f64) * t102701 + F::cast_from(0.69505208333333333334e-3_f64) * t7978 * t102328 + F::cast_from(0.23214722222222222222e-2_f64) * t102706 - F::cast_from(0.15476481481481481481e-2_f64) * t102709 + F::cast_from(0.69644166666666666666e-2_f64) * t102712 + F::cast_from(0.92858888888888888888e-2_f64) * t102715 - F::cast_from(0.46336805555555555556e-3_f64) * t27583 * t18128 * t27584 * t77753 - F::cast_from(0.82448622685185185187e-4_f64) * t99248 * t28701 + F::cast_from(0.15476481481481481481e-2_f64) * t102723 + t99646 + F::cast_from(0.23214722222222222221e-2_f64) * t102725;
    (t102725, t102727)
}
