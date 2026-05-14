//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1365/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1365<F: Float>(t123594: F, t6256: F, t112156: F, t112159: F, t112300: F, t123206: F, t123415: F, t123700: F, t127135: F, t127139: F, t127204: F, t127299: F, t127301: F, t127306: F, t127310: F, t127319: F, t127322: F, t127325: F, t14742: F, t28558: F, t28680: F, t31530: F, t54840: F, t55011: F) -> (F,) {
    let t127329 = t6256 * t123594;
    let t127337 = -0.16299066933744855968e0 * t127299 - 0.48327307107230638236e1 * t14742 * t127301 - 0.90613700826057446696e0 * t54840 * t127306 + 0.13592055123908617004e1 * t55011 * t127310 - 0.1611184118048991131e0 * t112156 * t127135 + 0.1611184118048991131e0 * t112159 * t127139 + 0.1611184118048991131e0 * t28558 * t123415 + 0.1611184118048991131e0 * t127319 + 0.13335600218518518519e0 * t127322 - 0.1611184118048991131e0 * t127325 - 0.4833552354146973393e0 * t28680 * t127204 + 0.11113000182098765433e-1 * t127329 - 0.16669500273148148149e-1 * t6256 * t123206 - 0.17780800291358024692e0 * t6256 * t123700 - 0.90613700826057446696e0 * t112300 * t31530;
    (t127337,)
}
