//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 677/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk677<F: Float>(t12526: F, t6915: F, t6914: F, t161: F, t165: F, t3116: F, t2488: F, t2487: F, t912: F, t587: F, t12381: F, t286: F, t708: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12527 = t6915 * t12526;
    let t12528 = t6914 * t12527;
    let t12531 = t161 * t165 * t3116;
    let t12532 = t2488 * t12531;
    let t12533 = t2487 * t12532;
    let t12535 = t912 * t12531;
    let t12536 = t587 * t12535;
    let t12538 = t912 * t12526;
    let t12539 = t587 * t12538;
    let t12541 = t2488 * t12526;
    let t12542 = t2487 * t12541;
    let t12555 = t12381 * t286 * t708;
    (t12527, t12528, t12531, t12532, t12533, t12535, t12536, t12538, t12539, t12541, t12542, t12555)
}
