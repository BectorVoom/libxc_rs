//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1303/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1303<F: Float>(t22587: F, t22623: F, t22642: F, t22681: F, t22734: F, t22772: F, t22856: F, t22898: F, t237: F, t3147: F, t6496: F, t22697: F, t22825: F, t22837: F, t22840: F, t22844: F, t22847: F, t22851: F, t22878: F, t22892: F, t22894: F, t3162: F, t6117: F) -> (F, F, F) {
    let t22902 = t237 * (t22587 + t22623 + t22642 + t22681 + t22734 + t22772 + t22856 + t22898);
    let t22904 = F::cast_from(0.51947577317044391277e2_f64) * t3147 * t6496;
    let t22910 = -t22825 + F::cast_from(0.19751673498613801407e-1_f64) * t237 * t22697 - F::cast_from(0.51947577317044391277e2_f64) * t6117 * t3162 + t22837 + t22840 + t22844 + t22847 + t22851 + t22878 + t22892 + t22894;
    (t22902, t22904, t22910)
}
