//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1139/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1139<F: Float>(t2333: F, t2892: F, t795: F, t10610: F, t3263: F, t10918: F, t3275: F, t9573: F, t11502: F, t40681: F, t11475: F, t11479: F, t3262: F) -> (F, F, F, F) {
    let t42453 = t2333 * t2892;
    let t42454 = t42453 * t795;
    let t42457 = F::new(3.0) / F::new(2.0) * t10610 * t3263 * t42454;
    let t42460 = t3275 * t10918 * t9573 / F::new(2.0);
    let t42462 = F::new(3.0) / F::new(2.0) * t40681 * t11502;
    let t42465 = F::new(3.0) / F::new(2.0) * t3262 * t11479 * t11475;
    (t42457, t42460, t42462, t42465)
}
