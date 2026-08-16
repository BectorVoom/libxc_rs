//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1007/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1007(t3115: f64, t3308: f64, t10776: f64, t3100: f64, t10772: f64, t10781: f64, t3105: f64, t261: f64, t3191: f64, t7628: f64, t3182: f64, t7614: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t12476 = t3308 * t3115;
    let t12477 = t10776 * t12476;
    let t12479 = t3308 * t3100;
    let t12480 = t10772 * t12479;
    let t12482 = t10781 * t3105;
    let t12486 = t261 * t3191;
    let t12487 = t7628 * t12486;
    let t12489 = t261 * t3182;
    let t12490 = t7614 * t12489;
    (t12476, t12477, t12479, t12480, t12482, t12486, t12487, t12489, t12490)
}
