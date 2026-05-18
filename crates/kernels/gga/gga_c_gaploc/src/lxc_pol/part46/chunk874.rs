//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 874/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk874<F: Float>(t10557: F, t30936: F, t1: F, t2392: F, t3338: F, t544: F, t594: F, t12987: F, t7014: F, t2365: F, t31558: F, t7025: F) -> (F, F, F, F) {
    let t42250 = F::new(0.17875244975925213335e2) * t10557 * t30936;
    let t42254 = t544 * t594 * t3338 * t1 * t2392;
    let t42256 = t7014 * t12987;
    let t42257 = F::new(0.15976219147466979032e-1) * t42256;
    let t42259 = t7025 * t2365 * t31558;
    (t42250, t42254, t42257, t42259)
}
