//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1235/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1235<F: Float>(t102889: F, t102892: F, t102894: F, t102896: F, t102898: F, t102900: F, t102902: F, t102904: F, t102906: F, t102908: F, t102910: F, t102937: F, t102939: F, t102942: F, t102944: F, t102946: F, t102948: F, t102950: F, t102952: F, t102954: F, t102956: F, t102958: F) -> (F, F) {
    let t103821 = t102889 / 6.0 + t102892 / 6.0 + t102894 / 48.0 + t102896 / 48.0 + t102898 / 128.0 - 2.0 / 3.0 * t102900 + t102902 / 144.0 - t102904 / 8.0 - t102906 / 12.0 - t102908 / 24.0 + 2.0 / 27.0 * t102910;
    let t103845 = -t102937 / 12.0 + 11.0 / 18.0 * t102939 + 11.0 / 27.0 * t102942 + t102944 / 64.0 - t102946 / 3.0 - t102948 / 12.0 - t102950 / 24.0 - t102952 / 48.0 - t102954 / 12.0 + t102956 / 8.0 - t102958 / 64.0;
    (t103821, t103845)
}
