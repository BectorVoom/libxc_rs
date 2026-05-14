//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 935/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk935<F: Float>(t5003: F, t997: F, t4547: F, t4754: F, t4535: F, t3037: F, t3210: F, t368: F, t398: F, t506: F, t3216: F, t4349: F, t4344: F, t1462: F, t3237: F, t4764: F) -> (F, F, F, F, F, F, F, F, F) {
    let t17661 = t997 * t5003;
    let t17663 = t997 * t4547;
    let t17669 = t997 * t4754;
    let t17671 = t997 * t4535;
    let t17681 = t3210 * t398 * t368 * t506 * t3037;
    let t17683 = t3216 * t4349;
    let t17687 = t997 * t4344;
    let t17689 = t3237 * t1462;
    let t17691 = t997 * t4764;
    (t17661, t17663, t17669, t17671, t17681, t17683, t17687, t17689, t17691)
}
