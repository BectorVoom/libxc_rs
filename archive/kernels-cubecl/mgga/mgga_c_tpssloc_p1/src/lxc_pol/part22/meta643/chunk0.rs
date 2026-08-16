//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2183/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2183<F: Float>(t5303: F, t53945: F, t16336: F, t5310: F, t1827: F, t54124: F, t16288: F, t5289: F, t19805: F, t68: F, t1340: F, t12365: F, t6417: F) -> (F, F, F, F, F, F, F) {
    let t56906 = t53945 * t5303;
    let t56909 = t16336 * t5310;
    let t56919 = t54124 * t1827;
    let t56921 = t16288 * t5289;
    let t56923 = t19805 * t68;
    let t56924 = t56923 * t1340;
    let t56927 = t12365 * t6417;
    (t56906, t56909, t56919, t56921, t56923, t56924, t56927)
}
