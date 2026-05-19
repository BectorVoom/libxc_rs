//! LDA_C_PMGB06 lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 373/1267 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pmgb06_lxc_pol_part5_v4rho4_3_chunk373<F: Float>(t1730: F, t206: F, t107: F, t1180: F, t290: F, t410: F, t701: F, t707: F, t711: F, t715: F, t301: F, t398: F, t413: F) -> (F, F, F, F, F, F) {
    let t1732 = F::cast_from(0.033245444444444446_f64) * t206 * t1730;
    let t1741 = F::cast_from(1.328721022894618_f64) * t107 * t1180 * t290;
    let t1743 = t107 * t410 * t701;
    let t1750 = t707 * t711;
    let t1753 = F::cast_from(0.039914113367515366_f64) * t707 * t715;
    let t1759 = t398 * t413 * t301;
    (t1732, t1741, t1743, t1750, t1753, t1759)
}
