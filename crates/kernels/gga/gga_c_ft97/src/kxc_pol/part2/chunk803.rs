//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 803/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk803<F: Float>(t1647: F, t3445: F, t2221: F, t3590: F, t558: F, t574: F, t2142: F, t3565: F, t144: F, t1053: F, t9428: F, t1882: F, t3480: F) -> (F, F, F, F, F, F, F) {
    let t12625 = t3445 * t1647;
    let t12626 = t2221 * t12625;
    let t12630 = t574 * t3590 * t558;
    let t12633 = t2142 * t3565;
    let t12634 = t144 * t12633;
    let t12637 = t9428 * t1053;
    let t12638 = t144 * t12637;
    let t12642 = F::new(2.0) / F::new(9.0) * t1882 * t3480;
    (t12626, t12630, t12633, t12634, t12637, t12638, t12642)
}
