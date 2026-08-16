//! MGGA_C_REVTPSS lxc pol — lxc_pol part 32 (v4rho3sigma_7) CSE chunk 1714/2056 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part32_v4rho3sigma_7_chunk1714<F: Float>(t27212: F, t786: F, t7063: F, t14685: F, t1941: F, t14756: F, t4435: F, t7045: F, t4426: F, t7038: F, t25245: F, t4430: F) -> (F, F, F, F, F, F, F) {
    let t27213 = t786 * t27212;
    let t27216 = t7063 * t27212;
    let t27221 = t1941 * t14685;
    let t27222 = t27221 * t14756;
    let t27224 = t7045 * t4435;
    let t27226 = t7038 * t4426;
    let t27228 = t25245 * t4430;
    (t27213, t27216, t27221, t27222, t27224, t27226, t27228)
}
