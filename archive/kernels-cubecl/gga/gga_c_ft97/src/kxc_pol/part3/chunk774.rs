//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 774/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk774<F: Float>(t11906: F, t3114: F, t3189: F, t1780: F, t971: F, t3195: F, t1851: F, t4551: F, t379: F, t1909: F, t4458: F, t447: F, t499: F) -> (F, F, F, F, F) {
    let t16024 = t11906 * t3114;
    let t16027 = t11906 * t3189;
    let t16030 = t1780 * t971;
    let t16031 = t16030 * t3195;
    let t16034 = t1851 * t4551;
    let t16035 = t16034 * t379;
    let t16036 = t1909 * t16035;
    let t16040 = t447 * t499 * t4458;
    (t16024, t16027, t16031, t16036, t16040)
}
