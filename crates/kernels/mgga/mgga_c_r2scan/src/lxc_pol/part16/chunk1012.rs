//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1012/1112 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1012<F: Float>(t12557: F, t3270: F, t3269: F, t10619: F, t12422: F, t12556: F, t498: F, t3275: F, t3352: F, t11559: F, t11629: F, t11004: F, t2333: F, t2892: F, t795: F, t10610: F, t3263: F) -> (F, F, F, F, F, F, F) {
    let t42439 = t3270 * t12557;
    let t42441 = t3269 * t42439 / 4.0;
    let t42443 = t12422 * t10619 / 4.0;
    let t42444 = t498 * t12556;
    let t42447 = t3275 * t42444 * t3352 / 4.0;
    let t42450 = 5.0 / 8.0 * t3275 * t11629 * t11559;
    let t42452 = 5.0 / 16.0 * t12422 * t11004;
    let t42453 = t2333 * t2892;
    let t42454 = t42453 * t795;
    let t42457 = 3.0 / 2.0 * t10610 * t3263 * t42454;
    (t42441, t42443, t42444, t42447, t42450, t42452, t42457)
}
