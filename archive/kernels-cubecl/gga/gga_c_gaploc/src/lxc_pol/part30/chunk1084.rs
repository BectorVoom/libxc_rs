//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1084/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1084<F: Float>(t1033: F, t5558: F, t1381: F, t2796: F, t1023: F, t5230: F, t5508: F, t1853: F, t2926: F, t24586: F, t2610: F, t1227: F, t3091: F) -> (F, F, F, F, F, F, F) {
    let t27229 = t1033 * t5558;
    let t27232 = t2796 * t1381;
    let t27348 = t1023 * t5230;
    let t27403 = t1023 * t5508;
    let t27661 = t2926 * t1853;
    let t27728 = t2610 * t24586;
    let t27835 = t3091 * t1227;
    (t27229, t27232, t27348, t27403, t27661, t27728, t27835)
}
