//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1273/1365 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};
use libxc_rkernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1273(t1325: f64, t22285: f64, t5256: f64, t519: f64, t7705: f64, t9723: f64, t5237: f64, t7643: f64, t22821: f64, t22824: f64, t22826: f64, t22828: f64, t22830: f64, t22833: f64, t22836: f64, t22839: f64, t22843: f64, t22844: f64) -> (f64, f64, f64, f64) {
    let t22847 = 8.0_f64 / 9.0_f64 * t1325 * t5256 * t22285;
    let t22849 = t519 * t9723 * t7705;
    let t22850 = 8.0_f64 / 27.0_f64 * t22849;
    let t22852 = t519 * t5237 * t7643;
    let t22853 = 8.0_f64 / 27.0_f64 * t22852;
    let t22854 = t22821 + t22824 - t22826 + t22828 + t22830 + t22833 - t22836 + t22839 + t22843 - t22844 + t22847 - t22850 + t22853;
    (t22847, t22850, t22853, t22854)
}
