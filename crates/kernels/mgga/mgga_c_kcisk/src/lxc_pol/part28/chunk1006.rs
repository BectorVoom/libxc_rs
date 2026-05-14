//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1006/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1006<F: Float>(t7274: F, t7283: F, t1894: F, t8786: F, t1899: F, t1873: F, t1869: F, t1757: F) -> (F, F, F, F) {
    let t23292 = t7283 * t7274;
    let t23299 = t8786 * t1894;
    let t23300 = t1899 * t23299;
    let t23301 = t1873 * t23300;
    let t23302 = t1869 * t23301;
    let t23304 = t8786 * t1757;
    (t23292, t23299, t23302, t23304)
}
