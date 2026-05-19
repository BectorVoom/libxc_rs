//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1017/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1017<F: Float>(t23514: F, t23545: F, t935: F, t915: F, t11387: F, t23466: F, t11385: F, t1642: F, t19049: F, t4719: F, t6223: F, t1699: F, t19153: F, t23448: F, t23450: F, t23455: F, t23459: F, t23461: F, t23463: F, t23465: F, t23469: F, t5023: F) -> (F, F, F, F, F) {
    let t23546 = t23514 + t23545;
    let t23547 = t23546 * t935;
    let t23549 = F::new(1.0) * t915 * t23547;
    let t23550 = t23466 * t11387;
    let t23552 = F::cast_from(0.51726012919273400301e3_f64) * t11385 * t23550;
    let t23554 = F::cast_from(0.17544670867903938621e1_f64) * t19049 * t1642;
    let t23556 = F::cast_from(0.17544670867903938621e1_f64) * t4719 * t6223;
    let t23560 = -F::new(3.0) * t1699 * t19153 * t5023 + t23448 - t23450 + t23455 - t23459 + t23461 + t23463 + t23465 - t23469 + t23549 + t23552 - t23554 - t23556;
    (t23549, t23552, t23554, t23556, t23560)
}
