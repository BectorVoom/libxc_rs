//! MGGA_C_REVTPSS lxc pol — lxc_pol part 28 (v4rho3sigma_3) CSE chunk 2169/2277 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part28_v4rho3sigma_3_chunk2169<F: Float>(t10778: F, t1941: F, t50538: F, t93016: F, t25222: F, t4435: F, t14868: F, t2661: F, t93082: F, t14751: F, t7045: F, t14757: F, t25234: F) -> (F, F, F, F, F, F) {
    let t99062 = t1941 * t10778;
    let t99063 = t99062 * t50538;
    let t99065 = F::cast_from(0.18071592998981862717e-4_f64) * t93016;
    let t99066 = t25222 * t4435;
    let t99069 = t2661 * t93082 * t14868;
    let t99070 = F::cast_from(0.57165357490759649296e-4_f64) * t99069;
    let t99071 = t7045 * t14751;
    let t99073 = t25234 * t14757;
    (t99063, t99065, t99066, t99070, t99071, t99073)
}
