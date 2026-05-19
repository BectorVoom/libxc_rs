//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1092/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1092<F: Float>(t30786: F, t30790: F, t1992: F, t5606: F, t7585: F, t7586: F, t1181: F, t4257: F, t604: F, t8463: F, t4791: F, t570: F) -> (F, F, F, F, F) {
    let t34986 = F::cast_from(0.21437009059034868486e-3_f64) * t30786;
    let t34987 = F::cast_from(0.28582678745379824648e-3_f64) * t30790;
    let t34990 = t7585 * t7586 * t1992 * t5606;
    let t34991 = F::cast_from(0.28582678745379824648e-3_f64) * t34990;
    let t34994 = t8463 * t1181 * t604 * t4257;
    let t34996 = t570 * t4791;
    (t34986, t34987, t34991, t34994, t34996)
}
