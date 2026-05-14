//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1324/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1324<F: Float>(t19333: F, t6386: F, t1882: F, t31729: F, t31854: F, t31843: F, t112992: F, t113007: F, t113009: F, t113017: F, t11593: F, t15133: F, t1901: F, t19868: F, t19873: F, t24890: F, t2862: F, t29302: F, t296: F, t4246: F, t4311: F, t446: F, t5409: F, t6365: F, t7036: F, t7045: F, t840: F, t99186: F) -> (F, F) {
    let t126162 = t19333 * t6386;
    let t126166 = t1882 * t31729;
    let t126181 = t1882 * t31854;
    let t126183 = t1882 * t31843;
    let t126197 = t112992 + 16.0 / 27.0 * t113007 + 4.0 / 27.0 * t113009 - t446 * t296 * t126162 / 3.0 + 2.0 / 9.0 * t126166 + t113017 + t446 * t840 * t19333 * t6365 / 3.0 + 2.0 / 9.0 * t1901 * t99186 * t5409 + 2.0 / 9.0 * t1901 * t24890 * t19868 - 4.0 / 9.0 * t11593 * t24890 * t19873 - t126181 / 9.0 - 2.0 / 9.0 * t126183 + 2.0 / 3.0 * t446 * t840 * t4246 * t29302 + 2.0 / 3.0 * t446 * t840 * t15133 * t7045 + 4.0 / 3.0 * t446 * t2862 * t4311 * t7036;
    (t126162, t126197)
}
