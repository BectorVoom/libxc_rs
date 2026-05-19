//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 417/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk417<F: Float>(t1691: F, t717: F, t720: F, t61: F, t424: F, t697: F, t1678: F, t614: F, t22: F, t263: F, t124: F, t5: F) -> (F, F, F, F, F, F, F) {
    let t1706 = t717 * t1691;
    let t1707 = t1706 * t720;
    let t1709 = F::cast_from(0.19263893255070628431e1_f64) * t61 * t1707;
    let t1710 = t424 * t697;
    let t1712 = t1678 * t614;
    let t1713 = t22 * t263;
    let t1714 = t1712 * t1713;
    let t1716 = t5 * t124;
    (t1707, t1709, t1710, t1712, t1713, t1714, t1716)
}
