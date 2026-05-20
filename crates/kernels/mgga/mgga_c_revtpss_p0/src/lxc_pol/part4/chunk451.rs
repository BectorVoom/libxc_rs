//! MGGA_C_REVTPSS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 451/1428 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part4_v3rho3_1_chunk451<F: Float>(t5: F, t1466: F, t1497: F, t603: F, t91: F, t117: F, t1468: F, t100: F, t55: F, t108: F, t105: F, t109: F, t97: F, tau1: F) -> (F, F, F, F, F, F, F) {
    let t7 = piecewise3::<F>(F::new(0.0) < t5, t5, -t5);
    let t8 = -t7 <= -F::cast_from(0.999999999999e0_f64);
    let t1501 = piecewise3::<F>(t8, F::new(0.0), t1466 * t91 - F::new(4.0) * t1497 * t603);
    let t1502 = t1501 * t117;
    let t1504 = t1468 / F::new(2.0);
    let t1505 = t100 * t1504;
    let t1507 = tau1 * t55;
    let t1509 = -t1504;
    let t1510 = t108 * t1509;
    let t1513 = F::new(5.0) / F::new(3.0) * t105 * t1510 - F::new(5.0) / F::new(3.0) * t1507 * t109 + F::new(5.0) / F::new(3.0) * t97 * t1505;
    (t1501, t1502, t1504, t1505, t1507, t1509, t1513)
}
