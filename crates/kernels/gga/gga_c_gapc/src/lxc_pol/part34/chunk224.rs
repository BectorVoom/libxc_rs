//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 224/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk224<F: Float>(t159: F, t751: F, t104: F, t260: F, t14: F, t1: F, t269: F, t546: F, t106: F, t257: F, t748: F, t10: F, t103: F, t164: F, t266: F, t303: F, t304: F, t758: F) -> (F, F, F, F, F, F, F, F) {
    let t849 = t751 * t159;
    let t852 = t260 * t104;
    let t853 = t852 * t14;
    let t854 = t269 * t1;
    let t855 = t854 * t546;
    let t858 = t106 * t257;
    let t859 = t858 * t748;
    let t868 = F::new(0.58998125e-2) * t849 * t304 - F::new(0.11799625e-1) * t853 * t855 - F::new(0.58998125e-2) * t303 * t859 - F::cast_from(0.14341111111111111111e-1_f64) * t103 * t10 * t266 - F::cast_from(0.21511666666666666667e-1_f64) * t103 * t164 * t758;
    (t849, t852, t853, t854, t855, t858, t859, t868)
}
