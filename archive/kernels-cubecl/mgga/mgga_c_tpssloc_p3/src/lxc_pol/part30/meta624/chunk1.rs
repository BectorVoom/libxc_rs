//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2025/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2025<F: Float>(t87066: F, t25245: F, t82031: F, t25251: F, t87049: F, t23012: F, t7529: F, t23110: F, t23185: F, t25241: F, t1484: F, t852: F) -> (F, F, F, F, F, F) {
    let t87067 = F::cast_from(0.38381794893125283518e-1_f64) * t87066;
    let t87068 = t82031 * t25245;
    let t87078 = t87049 * t25251;
    let t87080 = t23012 * t7529;
    let t87100 = t23185 * t23110 * t25241;
    let t87101 = F::cast_from(0.82246703342411321824e-2_f64) * t87100;
    let t87111 = t852 * t1484;
    (t87067, t87068, t87078, t87080, t87101, t87111)
}
