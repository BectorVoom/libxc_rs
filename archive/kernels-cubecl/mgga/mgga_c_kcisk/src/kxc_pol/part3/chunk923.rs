//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 923/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk923<F: Float>(t1248: F, t3583: F, t3979: F, t1237: F, t4037: F, t4046: F, t4054: F, t12983: F, t4065: F, t1249: F, t12868: F, t3118: F, t313: F, t353: F) -> (F, F, F, F, F, F) {
    let t13650 = t1248 * t3979 * t3583;
    let t13653 = t4037 * t1237 * t4046;
    let t13656 = t4054 * t1237 * t4046;
    let t13659 = t1248 * t4065 * t12983;
    let t13662 = t1248 * t1249 * t12868;
    let t13665 = t353 * t3118 * t313;
    (t13650, t13653, t13656, t13659, t13662, t13665)
}
