//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1784/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1784<F: Float>(t18285: F, t18297: F, t150: F, t190: F, t5944: F, t750: F, t189: F, t5825: F, t606: F, t4401: F, t10552: F, t10554: F, t14317: F, t18253: F, t18256: F, t18261: F, t18262: F, t18265: F, t18267: F, t18268: F, t1940: F, t2403: F, t4537: F, t4541: F, t4556: F, t775: F, t9278: F, t9308: F, t9316: F, t9329: F, t9333: F) -> (F, F, F, F, F, F, F, F) {
    let t18298 = t18285 + t18297;
    let t18299 = t150 * t18298;
    let t18300 = t18299 * t190;
    let t18301 = t5944 * t750;
    let t18305 = t189 * t5825;
    let t18306 = t18305 * t606;
    let t18308 = F::cast_from(12.0_f64) * t4401 * t18306;
    let t18309 = -F::cast_from(3.0_f64) * t18268 * t2403 * t775 - F::cast_from(2.0_f64) * t1940 * t4537 * t4556 + F::cast_from(12.0_f64) * t18253 * t4541 + F::cast_from(6.0_f64) * t18256 * t4541 - t10552 + t10554 + t14317 + t18261 + t18262 + t18265 + t18267 + t18300 + t18301 + t18308 - t9278 + t9308 + t9316 + t9329 + t9333;
    (t18298, t18299, t18300, t18301, t18305, t18306, t18308, t18309)
}
