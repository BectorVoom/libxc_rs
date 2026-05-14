//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1196/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1196<F: Float>(t33097: F, t7317: F, t7304: F, t9708: F, t7299: F, t9704: F, t33120: F, t5061: F) -> (F, F, F, F) {
    let t34362 = t33097 * t7317;
    let t34364 = t9708 * t7304;
    let t34366 = t9704 * t7299;
    let t34368 = t5061 * t33120;
    (t34362, t34364, t34366, t34368)
}
