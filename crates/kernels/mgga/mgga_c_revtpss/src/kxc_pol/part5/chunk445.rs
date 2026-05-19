//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 445/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk445<F: Float>(t114: F, t100: F, t1504: F, t55: F, t108: F, t105: F, t109: F, t97: F, t655: F, t653: F, t69: F, tau1: F) -> (F, F, F, F, F, F, F) {
    let t115 = F::new(1.0) < t114;
    let t1505 = t100 * t1504;
    let t1507 = tau1 * t55;
    let t1509 = -t1504;
    let t1510 = t108 * t1509;
    let t1513 = F::new(5.0) / F::new(3.0) * t105 * t1510 - F::new(5.0) / F::new(3.0) * t1507 * t109 + F::new(5.0) / F::new(3.0) * t97 * t1505;
    let t1514 = t655 * t1513;
    let t1518 = piecewise3::<F>(t115, F::new(0.0), -t653 - t69 * t1514 / F::new(8.0));
    (t1505, t1507, t1509, t1510, t1513, t1514, t1518)
}
