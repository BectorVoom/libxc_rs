//! GGA_C_GAPC lxc pol — lxc_pol part 33 (v4rho2sigma2_12) CSE chunk 347/1306 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part33_v4rho2sigma2_12_chunk347<F: Float>(t128: F, t1540: F, t21: F, t496: F, t138: F, t141: F, t1518: F, t1521: F, t1524: F, t1525: F, t1532: F, t1535: F, t1545: F, t488: F, t499: F, t502: F) -> (F, F) {
    let t1548 = t1540 * t128;
    let t1549 = t496 * t21;
    let t1552 = F::new(0.71188398362396778151e-1) * t1518 * t141 + F::new(0.15370222373699304374e-1) * t1521 * t488 - F::new(0.16179181445999267762e-2) * t1525 * t499 + F::new(0.28766584610986698081e-2) * t1525 * t502 - F::new(0.16179181445999267762e-3) * t1524 * t138 * t1532 + F::new(0.28766584610986698082e-3) * t1535 * t502 + F::new(0.16179181445999267762e-4) * t1540 * t138 * t1545 - F::new(0.28766584610986698082e-4) * t1548 * t1549;
    (t1548, t1552)
}
