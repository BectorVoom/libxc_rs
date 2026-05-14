//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1015/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1015<F: Float>(t31036: F, t9942: F, t1434: F, t193: F, t30859: F, t676: F, t27: F, t89: F, t24181: F, t4934: F, t1131: F, t27882: F, t5053: F, t6008: F, t24538: F, t27811: F, t27826: F, t27873: F, t27876: F, t31011: F, t31017: F, t31022: F, t31027: F, t31032: F, t31034: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t31037 = t9942 * t31036;
    let t31039 = t1434 * t193 * t31037;
    let t31041 = t676 * t30859;
    let t31043 = t89 * t27 * t31041;
    let t31044 = t24181 * t4934;
    let t31045 = t193 * t31044;
    let t31046 = t89 * t31045;
    let t31048 = t27882 * t1131;
    let t31049 = t193 * t31048;
    let t31050 = t89 * t31049;
    let t31052 = t6008 * t5053;
    let t31053 = t193 * t31052;
    let t31054 = t89 * t31053;
    let t31059 = 2.0 / 3.0 * t31011 - t24538 + 2.0 / 3.0 * t27811 + 2.0 * t31017 + t31022 / 4.0 + t31027 / 2.0 + t31032 - 4.0 / 3.0 * t31034 - 3.0 * t31039 - t31043 - 6.0 * t31046 + 4.0 * t31050 + 2.0 * t31054 - 4.0 / 3.0 * t27826 - t27873 / 6.0 - 2.0 / 3.0 * t27876;
    (t31037, t31039, t31041, t31043, t31044, t31046, t31048, t31050, t31052, t31054, t31059)
}
