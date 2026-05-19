//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1073/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1073<F: Float>(t1835: F, t87: F, t5829: F, t690: F, t1731: F, t218: F, t220: F, t5555: F, t679: F, t16194: F, t213: F, t778: F) -> (F, F, F, F, F, F) {
    let t17359 = t1835 * t1835;
    let t17361 = F::new(1.0) / t87 / t17359;
    let t17391 = t690 * t5829;
    let t17402 = t218 * t1731 * t220;
    let t17403 = F::cast_from(0.13490888888888888889e1_f64) * t17402;
    let t17405 = t218 * t5555 * t679;
    let t17432 = F::new(1.0) / t213 / t16194 / t778 / F::new(96.0);
    (t17361, t17391, t17402, t17403, t17405, t17432)
}
