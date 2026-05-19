//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1146/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk1146<F: Float>(t5710: F, t72: F, t1432: F, t686: F, t136: F, t1892: F, t2457: F, t3964: F, t2435: F, t5760: F, t3999: F, t545: F) -> (F, F, F, F, F) {
    let t14155 = t5710 * t72;
    let t14158 = F::cast_from(0.19514881078765566038e-1_f64) * t1432 * t14155 * t686;
    let t14159 = t1892 * t136;
    let t14161 = t3964 * t14159 * t2457;
    let t14166 = t2435 * t5760;
    let t14171 = t3999 * t1892;
    let t14188 = t545 * t5710;
    (t14158, t14161, t14166, t14171, t14188)
}
