//! GGA_C_OPTC lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 831/1451 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part11_v4rho4_4_chunk831<F: Float>(t1220: F, t16094: F, t4230: F, t4539: F, t1570: F, t4275: F, t1199: F, t5454: F, t12966: F, t1256: F, t4599: F) -> (F, F, F, F, F, F) {
    let t16095 = t1220 * t16094;
    let t16097 = t4230 * t4539;
    let t16099 = t1570 * t4275;
    let t16135 = t5454 * t1199;
    let t16220 = F::new(12.0) * t12966;
    let t16221 = t4599 * t1256;
    (t16095, t16097, t16099, t16135, t16220, t16221)
}
