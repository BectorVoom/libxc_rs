//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1273/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1273<F: Float>(t1325: F, t22285: F, t5256: F, t519: F, t7705: F, t9723: F, t5237: F, t7643: F, t22821: F, t22824: F, t22826: F, t22828: F, t22830: F, t22833: F, t22836: F, t22839: F, t22843: F, t22844: F) -> (F, F, F, F) {
    let t22847 = F::new(8.0) / F::new(9.0) * t1325 * t5256 * t22285;
    let t22849 = t519 * t9723 * t7705;
    let t22850 = F::new(8.0) / F::new(27.0) * t22849;
    let t22852 = t519 * t5237 * t7643;
    let t22853 = F::new(8.0) / F::new(27.0) * t22852;
    let t22854 = t22821 + t22824 - t22826 + t22828 + t22830 + t22833 - t22836 + t22839 + t22843 - t22844 + t22847 - t22850 + t22853;
    (t22847, t22850, t22853, t22854)
}
