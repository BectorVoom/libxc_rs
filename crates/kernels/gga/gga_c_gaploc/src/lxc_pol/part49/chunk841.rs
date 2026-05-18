//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 841/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk841<F: Float>(t10627: F, t22623: F, t24885: F, t787: F, t1457: F, t2634: F, t2610: F, t7291: F, t10667: F, t2089: F, t321: F, t3431: F) -> (F, F, F, F, F, F) {
    let t32847 = t22623 * t10627;
    let t32969 = t787 * t24885;
    let t32970 = t1457 * t2634;
    let t33087 = t2610 * t7291;
    let t33118 = t2089 * t10667;
    let t33137 = t321 * t3431;
    (t32847, t32969, t32970, t33087, t33118, t33137)
}
