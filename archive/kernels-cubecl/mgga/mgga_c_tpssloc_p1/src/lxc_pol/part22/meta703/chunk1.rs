//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 2291/2721 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk2291<F: Float>(t15572: F, t15740: F, t11697: F, t18382: F, t3577: F, t1215: F, t6224: F, t1227: F, t13969: F, t18954: F, t19067: F, t1222: F, t18297: F) -> (F, F, F, F, F, F) {
    let t66360 = t15740 * t15572;
    let t66363 = t3577 * t11697 * t18382;
    let t66388 = t6224 * t1215;
    let t66398 = t1227 * t13969 * t18954;
    let t66406 = t1227 * t13969 * t19067;
    let t66408 = t18297 * t1222;
    (t66360, t66363, t66388, t66398, t66406, t66408)
}
