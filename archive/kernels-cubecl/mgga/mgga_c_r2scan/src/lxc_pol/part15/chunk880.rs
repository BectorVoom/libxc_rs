//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 880/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk880<F: Float>(t495: F, t8001: F, t360: F, t1551: F, t2567: F, t1567: F, t2530: F, t2124: F, t2591: F, t2590: F, t6127: F, t259: F, t6203: F) -> (F, F, F, F, F, F, F) {
    let t8002 = t8001 * t495;
    let t8003 = t360 * t8002;
    let t8006 = t2567 * t1551;
    let t8007 = t360 * t8006;
    let t8012 = t1567 * t2530;
    let t8014 = t2124 * t8012 * t2591;
    let t8018 = t2124 * t2590 * t6127;
    let t8021 = t6203 * t259;
    (t8002, t8003, t8006, t8007, t8014, t8018, t8021)
}
