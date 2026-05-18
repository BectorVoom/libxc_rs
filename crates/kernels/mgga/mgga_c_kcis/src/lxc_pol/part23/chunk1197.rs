//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1197/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1197<F: Float>(t12832: F, t27625: F, t7978: F, t27641: F, t4425: F, t94588: F, t12844: F, t27583: F, t27585: F, t94904: F, t7968: F, t95006: F) -> (F, F, F, F, F, F, F) {
    let t95045 = t7978 * t12832 * t27625;
    let t95052 = t7978 * t4425 * t27641;
    let t95088 = F::new(0.51588271604938271604e-3) * t94588;
    let t95115 = t27583 * t12844 * t27585;
    let t95123 = t7978 * t94904;
    let t95125 = t7968 * t94904;
    let t95127 = t7978 * t95006;
    (t95045, t95052, t95088, t95115, t95123, t95125, t95127)
}
