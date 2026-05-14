//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 606/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk606<F: Float>(t776: F, t41: F, t4794: F, t1758: F, t1995: F, t4973: F, t4977: F, t525: F, t642: F, t773: F, t79: F, t781: F, t2005: F, t2009: F, t2004: F, t1772: F, sigma2: F) -> (F, F, F, F, F, F, F) {
    let t777 = t776 < -0.66725e-1;
    let t5449 = t4794 * t41;
    let t5463 = piecewise3(t777, 0.0, 10.0 / 9.0 * t525 * t5449 * t642 - 20.0 / 27.0 * t525 * t1995 * t1758 + 40.0 / 81.0 * t525 * t773 * t4973 - 10.0 / 27.0 * t525 * t773 * t4977);
    let t5464 = t79 * t5463;
    let t5465 = t5464 * t781;
    let t5468 = t2005 * t2009;
    let t5470 = t2004 * sigma2;
    let t5471 = t5470 * t1772;
    (t5449, t5463, t5464, t5465, t5468, t5470, t5471)
}
