//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 913/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk913<F: Float>(t322: F, t1339: F, t352: F, t1338: F, t2441: F, t1035: F, t6755: F, t8397: F, t1348: F, t6767: F, t1018: F, t1307: F, t2405: F, t2437: F, t2438: F, t2445: F, t330: F, t6751: F, t837: F, t8420: F, t8425: F, t8454: F, t8479: F, t855: F) -> (F, F, F) {
    let t323 = t322 <= F::new(0.0);
    let t331 = t322 <= F::new(0.25e1);
    let t332 = F::new(0.25e1) < t322;
    let t8481 = t352 * t1339;
    let t8484 = t1338 * t2441;
    let t8487 = t6755 * t1035;
    let t8492 = piecewise3::<f64>(t332, t8397, F::new(0.0));
    let t8496 = t1348 * t2441;
    let t8501 = t6767 * t1035;
    let t8505 = piecewise5::<f64>(t323, t1018 * t1307 * t330 + F::new(2.0) * t2405 * t837 * t330 + t8420 * t330 + t8425 * t330, t331, t8454 + t8479, -F::new(0.63e1) * t2445 * t8481 - F::new(0.42e1) * t8484 * t2438 - F::new(0.945e1) * t8487 * t8481 - F::new(0.21e1) * t2437 * t6751 - F::new(0.105e1) * t855 * t8492 * t352 - F::new(0.315e1) * t8496 * t2438 - F::new(0.1575e1) * t2445 * t6751 - F::new(0.23625e1) * t8501 * t8481);
    (t8481, t8492, t8505)
}
