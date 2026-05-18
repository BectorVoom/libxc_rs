//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 976/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk976<F: Float>(t322: F, t1338: F, t3416: F, t1096: F, t6755: F, t11059: F, t1348: F, t6767: F, t1079: F, t11082: F, t11087: F, t11117: F, t11141: F, t1307: F, t2438: F, t330: F, t3381: F, t3413: F, t3420: F, t352: F, t6751: F, t837: F, t8481: F, t855: F) -> (F, F, F, F, F, F) {
    let t323 = t322 <= F::new(0.0);
    let t331 = t322 <= F::new(0.25e1);
    let t332 = F::new(0.25e1) < t322;
    let t11145 = t1338 * t3416;
    let t11148 = t6755 * t1096;
    let t11153 = piecewise3::<f64>(t332, t11059, F::new(0.0));
    let t11157 = t1348 * t3416;
    let t11162 = t6767 * t1096;
    let t11166 = piecewise5::<f64>(t323, t1079 * t1307 * t330 + F::new(2.0) * t3381 * t837 * t330 + t11082 * t330 + t11087 * t330, t331, t11117 + t11141, -F::new(0.63e1) * t3420 * t8481 - F::new(0.42e1) * t11145 * t2438 - F::new(0.945e1) * t11148 * t8481 - F::new(0.21e1) * t3413 * t6751 - F::new(0.105e1) * t855 * t11153 * t352 - F::new(0.315e1) * t11157 * t2438 - F::new(0.1575e1) * t3420 * t6751 - F::new(0.23625e1) * t11162 * t8481);
    (t11145, t11148, t11153, t11157, t11162, t11166)
}
