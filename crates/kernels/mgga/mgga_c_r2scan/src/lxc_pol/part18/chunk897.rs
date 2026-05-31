//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 897/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk897<F: Float>(t2266: F, t481: F, t9589: F, t2900: F, t6621: F, t806: F, t35: F, t990: F, t1216: F, t1248: F, t2904: F, t4911: F) -> (F, F, F, F, F) {
    let t9591 = t2266 * t9589 * t481;
    let t9592 = F::cast_from(3.0_f64) * t9591;
    let t9597 = t6621 * t2900;
    let t9598 = t9597 * t806;
    let t9601 = t990 * t35;
    let t9602 = t9601 * t1216;
    let t9607 = t1248 * t2904;
    let t9608 = t9607 * t806;
    let t9612 = -t1216 - F::cast_from(3.0_f64) * t4911;
    (t9592, t9598, t9602, t9608, t9612)
}
