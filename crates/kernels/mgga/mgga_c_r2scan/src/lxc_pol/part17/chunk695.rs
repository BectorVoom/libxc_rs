//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 695/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk695<F: Float>(t1830: F, t695: F, t5434: F, t61: F, t717: F, t1719: F, t749: F, t1883: F, t625: F, t626: F, t630: F, t648: F) -> (F, F, F, F, F, F) {
    let t5435 = t1830 * t695;
    let t5437 = F::cast_from(0.3903689268108626343e0_f64) * t5434 * t5435;
    let t5438 = t61 * t717;
    let t5439 = t749 * t1719;
    let t5441 = F::cast_from(0.57791679765211885293e1_f64) * t5438 * t5439;
    let t5444 = F::cast_from(0.53424999999999999999e-1_f64) * t625 * t626 * t1883;
    let t5446 = F::new(1.0) / t648 / t630;
    (t5435, t5437, t5439, t5441, t5444, t5446)
}
