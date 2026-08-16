//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 741/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk741<F: Float>(t122: F, t2111: F, t409: F, t57: F, t128: F, t494: F, t538: F, t113: F, t2: F, t386: F, t1567: F, t774: F) -> (F, F, F, F, F, F, F) {
    let t6188 = t2111 * t122;
    let t6189 = t409 * t57;
    let t6190 = t6189 * t128;
    let t6191 = t6188 * t6190;
    let t6192 = t538 * t494;
    let t6194 = t113 * t2 * t386;
    let t6195 = t6192 * t6194;
    let t6196 = t6191 * t6195;
    let t6203 = t1567 * t774;
    (t6188, t6189, t6190, t6191, t6194, t6196, t6203)
}
