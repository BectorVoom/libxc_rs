//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 1387/1397 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk1387(t102889: f64, t102892: f64, t102894: f64, t102896: f64, t102898: f64, t102900: f64, t102902: f64, t102904: f64, t102906: f64, t102908: f64, t102910: f64, t102937: f64, t102939: f64, t102942: f64, t102944: f64, t102946: f64, t102948: f64, t102950: f64, t102952: f64, t102954: f64, t102956: f64, t102958: f64) -> (f64, f64) {
    let t103821 = t102889 / 6.0_f64 + t102892 / 6.0_f64 + t102894 / 48.0_f64 + t102896 / 48.0_f64 + t102898 / 128.0_f64 - 2.0_f64 / 3.0_f64 * t102900 + t102902 / 144.0_f64 - t102904 / 8.0_f64 - t102906 / 12.0_f64 - t102908 / 24.0_f64 + 2.0_f64 / 27.0_f64 * t102910;
    let t103845 = -t102937 / 12.0_f64 + 11.0_f64 / 18.0_f64 * t102939 + 11.0_f64 / 27.0_f64 * t102942 + t102944 / 64.0_f64 - t102946 / 3.0_f64 - t102948 / 12.0_f64 - t102950 / 24.0_f64 - t102952 / 48.0_f64 - t102954 / 12.0_f64 + t102956 / 8.0_f64 - t102958 / 64.0_f64;
    (t103821, t103845)
}
