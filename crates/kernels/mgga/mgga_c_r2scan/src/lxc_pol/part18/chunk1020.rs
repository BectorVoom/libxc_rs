//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1020/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1020<F: Float>(t1018: F, t1079: F, t11920: F, t12622: F, t12624: F, t12627: F, t12629: F, t2405: F, t2406: F, t2951: F, t2953: F, t330: F, t3381: F, t3643: F, t3645: F, t42559: F, t42592: F, t837: F, t838: F, t9698: F) -> (F,) {
    let t42615 = (t42559 + t42592) * t330 + t12622 * t837 * t330 + 2.0 * t11920 * t1018 * t330 + 2.0 * t3643 * t2405 * t330 + 2.0 * t12624 * t838 + t3381 * t2951 * t330 + t1079 * t9698 * t330 + t12627 * t838 + t3381 * t2953 * t330 + 2.0 * t3645 * t2406 + t12629 * t838;
    (t42615,)
}
