//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1926/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1926<F: Float>(t14693: F, t25270: F, t14927: F, t27261: F, t10778: F, t1941: F, t50538: F, t25222: F, t4435: F, t14868: F, t2661: F, t93082: F) -> (F, F, F, F, F) {
    let t99054 = t25270 * t14693;
    let t99056 = t27261 * t14927;
    let t99062 = t1941 * t10778;
    let t99063 = t99062 * t50538;
    let t99066 = t25222 * t4435;
    let t99069 = t2661 * t93082 * t14868;
    (t99054, t99056, t99063, t99066, t99069)
}
