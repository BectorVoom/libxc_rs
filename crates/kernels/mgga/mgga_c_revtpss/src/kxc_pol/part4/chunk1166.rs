//! MGGA_C_REVTPSS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1166/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part4_v3rho3_1_chunk1166<F: Float>(t45: F, t11064: F, t1583: F, t1469: F, t2609: F, t706: F, t10593: F, t10597: F, t4186: F, t80: F, t13312: F, t1490: F, t2251: F, t2258: F, t4328: F, t606: F, t766: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t151 = t45 <= zeta_threshold;
    let t14436 = t1583 * t11064;
    let t14440 = t2609 * t1469;
    let t14441 = t706 * t14440;
    let t14442 = F::new(4.0) * t14441;
    let t14443 = F::new(0.11696447245269292414e1) * t10593;
    let t14444 = F::new(0.34631718211362927518e2) * t10597;
    let t14447 = t80 * t4186;
    let t14455 = piecewise3::<f64>(t151, F::new(0.0), F::new(8.0) / F::new(27.0) * t1490 * t2251 - F::new(4.0) / F::new(9.0) * t14447 * t606 - F::new(2.0) / F::new(9.0) * t4328 * t2258 + F::new(2.0) / F::new(3.0) * t766 * t13312);
    (t14436, t14442, t14443, t14444, t14455)
}
