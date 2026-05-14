//! GGA_C_GAPC lxc pol — lxc_pol part 25 (v4rho2sigma2_4) CSE chunk 342/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part25_v4rho2sigma2_4_chunk342<F: Float>(t1524: F, t493: F, t124: F, t4: F, t495: F, t128: F, t511: F, t8: F, t134: F, t122: F, t186: F, t21: F, t496: F, t138: F, t141: F, t1518: F, t1521: F, t488: F, t499: F, t502: F) -> (F, F, F, F, F, F, F, F) {
    let t1525 = t1524 * t493;
    let t1532 = t495 * t124 * t4;
    let t1535 = t1524 * t128;
    let t1539 = 1.0 / t8 / t511;
    let t1540 = t1539 * t134;
    let t1543 = 1.0 / t186 / t122;
    let t1545 = t1543 * t124 * t21;
    let t1548 = t1540 * t128;
    let t1549 = t496 * t21;
    let t1552 = 0.71188398362396778151e-1 * t1518 * t141 + 0.15370222373699304374e-1 * t1521 * t488 - 0.16179181445999267762e-2 * t1525 * t499 + 0.28766584610986698081e-2 * t1525 * t502 - 0.16179181445999267762e-3 * t1524 * t138 * t1532 + 0.28766584610986698082e-3 * t1535 * t502 + 0.16179181445999267762e-4 * t1540 * t138 * t1545 - 0.28766584610986698082e-4 * t1548 * t1549;
    (t1525, t1532, t1535, t1539, t1540, t1545, t1548, t1552)
}
