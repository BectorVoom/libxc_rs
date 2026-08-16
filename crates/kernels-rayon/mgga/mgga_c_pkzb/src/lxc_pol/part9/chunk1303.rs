//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 1303/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk1303(t22587: f64, t22623: f64, t22642: f64, t22681: f64, t22734: f64, t22772: f64, t22856: f64, t22898: f64, t237: f64, t3147: f64, t6496: f64, t22697: f64, t22825: f64, t22837: f64, t22840: f64, t22844: f64, t22847: f64, t22851: f64, t22878: f64, t22892: f64, t22894: f64, t3162: f64, t6117: f64) -> (f64, f64, f64) {
    let t22902 = t237 * (t22587 + t22623 + t22642 + t22681 + t22734 + t22772 + t22856 + t22898);
    let t22904 = 0.51947577317044391277e2_f64 * t3147 * t6496;
    let t22910 = -t22825 + 0.19751673498613801407e-1_f64 * t237 * t22697 - 0.51947577317044391277e2_f64 * t6117 * t3162 + t22837 + t22840 + t22844 + t22847 + t22851 + t22878 + t22892 + t22894;
    (t22902, t22904, t22910)
}
