//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 984/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk984<F: Float>(t14443: F, t14510: F, t467: F, t488: F, t1501: F, t4306: F, t13320: F, t4231: F, t4230: F, t13394: F, t6317: F, t6316: F, sigma0: F) -> (F, F, F, F) {
    let t14511 = t14443 + t14510;
    let t14512 = t14511 * t467;
    let t14513 = t14512 * sigma0;
    let t14514 = t14513 * t488;
    let t14516 = t1501 * t4306;
    let t14518 = t4231 * t13320;
    let t14519 = t4230 * t14518;
    let t14521 = t6317 * t13394;
    let t14522 = t6316 * t14521;
    (t14514, t14516, t14519, t14522)
}
