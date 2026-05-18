//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 970/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk970<F: Float>(t11336: F, t3270: F, t795: F, t3269: F, t1115: F, t481: F, t10667: F, t3493: F, t10630: F, t3262: F, t3465: F, t11020: F, t3469: F) -> (F, F, F, F, F, F, F, F) {
    let t11338 = t3270 * t11336 * t795;
    let t11339 = t3269 * t11338;
    let t11340 = t11339 / F::new(2.0);
    let t11342 = t3270 * t1115 * t481;
    let t11343 = t10667 * t11342;
    let t11344 = F::new(3.0) / F::new(2.0) * t11343;
    let t11345 = t3270 * t3493;
    let t11346 = t3269 * t11345;
    let t11347 = t11346 / F::new(2.0);
    let t11349 = t3262 * t3465 * t10630;
    let t11350 = F::new(3.0) / F::new(4.0) * t11349;
    let t11351 = t11020 * t3469;
    (t11338, t11340, t11342, t11344, t11345, t11347, t11350, t11351)
}
