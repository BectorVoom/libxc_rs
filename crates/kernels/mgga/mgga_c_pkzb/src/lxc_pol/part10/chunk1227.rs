//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 1227/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk1227<F: Float>(t19523: F, t19530: F, t23729: F, t23732: F, t23739: F, t23743: F, t23762: F, t23767: F, t23773: F, t23776: F, t23782: F, t23786: F, t23793: F, t34: F, t38: F, t454: F, t6676: F, t6683: F, t6689: F, t8621: F, t8631: F, t991: F) -> (F,) {
    let t23855 = -40.0 / 27.0 * t19523 * t23732 + 40.0 / 27.0 * t19530 * t23729 + 100.0 / 81.0 * t991 * t6676 - 50.0 / 3.0 * t991 * t6689 + 40.0 / 81.0 * t38 * t23762 - 20.0 / 9.0 * t38 * t23739 - 10.0 / 27.0 * t38 * t23743 + 10.0 / 9.0 * t38 * t23793 + 100.0 / 81.0 * t454 * t8621 - 10.0 / 27.0 * t34 * t23767 - 100.0 / 27.0 * t454 * t8631 + 20.0 / 9.0 * t34 * t23773 + 10.0 / 9.0 * t34 * t23776 - 100.0 / 27.0 * t991 * t6683 - 10.0 / 27.0 * t38 * t23782 + 20.0 / 9.0 * t38 * t23786;
    (t23855,)
}
