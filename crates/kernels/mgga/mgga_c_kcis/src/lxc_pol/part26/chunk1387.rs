//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1387/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1387<F: Float>(t102889: F, t102892: F, t102894: F, t102896: F, t102898: F, t102900: F, t102902: F, t102904: F, t102906: F, t102908: F, t102910: F, t102937: F, t102939: F, t102942: F, t102944: F, t102946: F, t102948: F, t102950: F, t102952: F, t102954: F, t102956: F, t102958: F) -> (F, F) {
    let t103821 = t102889 / F::new(6.0) + t102892 / F::new(6.0) + t102894 / F::new(48.0) + t102896 / F::new(48.0) + t102898 / F::new(128.0) - F::new(2.0) / F::new(3.0) * t102900 + t102902 / F::new(144.0) - t102904 / F::new(8.0) - t102906 / F::new(12.0) - t102908 / F::new(24.0) + F::new(2.0) / F::new(27.0) * t102910;
    let t103845 = -t102937 / F::new(12.0) + F::new(11.0) / F::new(18.0) * t102939 + F::new(11.0) / F::new(27.0) * t102942 + t102944 / F::new(64.0) - t102946 / F::new(3.0) - t102948 / F::new(12.0) - t102950 / F::new(24.0) - t102952 / F::new(48.0) - t102954 / F::new(12.0) + t102956 / F::new(8.0) - t102958 / F::new(64.0);
    (t103821, t103845)
}
