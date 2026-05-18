//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 1010/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk1010<F: Float>(t35646: F, t171: F, t5011: F, t2310: F, t7780: F, t31643: F, t527: F, t1418: F, t7605: F, t1347: F, t1980: F, t35383: F, t7458: F) -> (F, F, F, F, F, F, F) {
    let t35647 = F::new(0.1528125e-1) * t35646;
    let t35649 = t171 * t5011;
    let t35662 = t7780 * t2310;
    let t35664 = t31643 * t527;
    let t35672 = t7605 * t1418;
    let t35673 = F::new(0.68598428988911579156e-2) * t35672;
    let t35678 = t7605 * t1347;
    let t35679 = F::new(0.68598428988911579156e-2) * t35678;
    let t35682 = t1980 * t7458 * t35383;
    (t35647, t35649, t35662, t35664, t35673, t35679, t35682)
}
