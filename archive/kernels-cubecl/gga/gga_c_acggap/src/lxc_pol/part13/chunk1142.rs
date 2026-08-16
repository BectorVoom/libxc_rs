//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1142/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1142<F: Float>(t2001: F, t5255: F, t5003: F, t1418: F, t7605: F, t5260: F, t4547: F, t1347: F, t1980: F, t35383: F, t7458: F, t31773: F, t8634: F) -> (F, F, F, F, F, F, F, F) {
    let t35668 = t2001 * t5255;
    let t35670 = t2001 * t5003;
    let t35672 = t7605 * t1418;
    let t35673 = F::cast_from(0.68598428988911579156e-2_f64) * t35672;
    let t35674 = t2001 * t5260;
    let t35676 = t2001 * t4547;
    let t35678 = t7605 * t1347;
    let t35679 = F::cast_from(0.68598428988911579156e-2_f64) * t35678;
    let t35682 = t1980 * t7458 * t35383;
    let t35683 = F::cast_from(0.28582678745379824648e-3_f64) * t35682;
    let t35685 = t31773 * t8634;
    (t35668, t35670, t35673, t35674, t35676, t35679, t35683, t35685)
}
