//! GGA_C_FT97 kxc pol — kxc_pol part 2 (v3rho3_1) CSE chunk 583/869 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part2_v3rho3_1_chunk583<F: Float>(t2: F, t8326: F, t1775: F, t1788: F, t1793: F, t369: F, t631: F, t637: F, t7242: F, t96: F, t1841: F, t487: F, t1786: F, t480: F, t375: F, t443: F, t444: F) -> (F, F, F, F, F, F, F) {
    let t8327 = t8326 * t2;
    let t8331 = t1775 * t1788;
    let t8333 = t1775 * t1793;
    let t8345 = 1.0 / t96 / t631 / t637 / t369 / t7242 / 4.0;
    let t8360 = t1841 * t487;
    let t8372 = t1786 * t480;
    let t8392 = t443 * t444 * t375;
    (t8327, t8331, t8333, t8345, t8360, t8372, t8392)
}
