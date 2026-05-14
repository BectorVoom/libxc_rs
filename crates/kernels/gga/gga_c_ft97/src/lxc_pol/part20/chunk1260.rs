//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1260/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1260<F: Float>(t1248: F, t25183: F, t2843: F, t1882: F, t29304: F, t29321: F, t29325: F, t29124: F, t8392: F, t29281: F, t7051: F, t8232: F, t15128: F, t15369: F, t15433: F, t15485: F, t1901: F, t25266: F, t25271: F, t296: F, t446: F, t6353: F, t840: F, t99009: F, t99016: F, t99025: F, t99030: F, t99032: F) -> (F, F) {
    let t113950 = t2843 * t25183 * t1248;
    let t113956 = 2.0 / 9.0 * t1882 * t29304;
    let t113968 = 4.0 / 9.0 * t1882 * t29321;
    let t113972 = 4.0 / 9.0 * t1882 * t29325;
    let t113974 = 4.0 / 9.0 * t8392 * t29124;
    let t113980 = 2.0 / 9.0 * t1882 * t29281;
    let t113981 = t8232 * t7051;
    let t113983 = 2.0 / 3.0 * t446 * t296 * t113950 - 4.0 / 9.0 * t99009 - t113956 + t446 * t840 * t6353 * t15433 / 3.0 + 2.0 / 27.0 * t99016 - t99025 / 9.0 - 2.0 / 3.0 * t446 * t840 * t15128 * t25266 - t113968 + 16.0 / 27.0 * t99030 + 8.0 / 27.0 * t99032 - t113972 + t113974 + 4.0 / 3.0 * t1901 * t15369 * t25271 * t15485 + t113980 - 4.0 / 27.0 * t113981;
    (t113950, t113983)
}
