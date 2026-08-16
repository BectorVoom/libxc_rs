//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 32 (v4rho3sigma_8) CSE chunk 2326/2369 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part32_v4rho3sigma_8_chunk2326(t27634: f64, t3030: f64, t95420: f64, t52: f64, t6144: f64, t24682: f64, t460: f64, t1210: f64, t1215: f64, t18387: f64, t18969: f64, t24741: f64, t27639: f64, t27645: f64, t29563: f64, t3032: f64, t475: f64, t488: f64, t4965: f64, t6224: f64, t7321: f64, t7331: f64, t8048: f64, t86275: f64, t86278: f64, t95396: f64, t95480: f64, t95487: f64, t95491: f64) -> (f64, f64) {
    let t104266 = t27634 * t95420 * t3030;
    let t104280 = t52 * t6144;
    let t104282 = t24682 * t104280 * t460;
    let t104292 = -0.40372756094140390856e-3_f64 * t104266 * t27639 + 0.20186378047070195428e-3_f64 * t104266 * t27645 + t95480 - 0.72670960969452703541e-2_f64 * t29563 * t7321 - t95487 - t86275 / 6912.0_f64 + 0.10093189023535097714e-3_f64 * t95396 * t1210 * t6224 * t3032 * t1215 * t475 - 0.10093189023535097714e-3_f64 * t104282 * t7331 + t86278 - t95491 - t4965 * t8048 * t488 / 144.0_f64 - t24741 * t18969 / 2304.0_f64 - t24741 * t18387 / 1152.0_f64;
    (t104280, t104292)
}
