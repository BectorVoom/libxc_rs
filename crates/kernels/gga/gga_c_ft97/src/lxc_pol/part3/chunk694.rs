//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 694/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk694<F: Float>(t1852: F, t463: F, t110: F, t8216: F, t1882: F, t3210: F, t8232: F, t951: F, t3216: F, t1786: F, t971: F, t3184: F, t8392: F) -> (F, F, F, F, F, F, F, F) {
    let t11854 = t463 * t1852;
    let t11863 = t8216 * t110;
    let t11882 = F::new(2.0) / F::new(27.0) * t1882 * t3210;
    let t11883 = t8232 * t951;
    let t11897 = F::new(2.0) / F::new(9.0) * t1882 * t3216;
    let t11902 = t1786 * t971;
    let t11906 = t463 * t971;
    let t11913 = F::new(2.0) / F::new(27.0) * t8392 * t3184;
    (t11854, t11863, t11882, t11883, t11897, t11902, t11906, t11913)
}
