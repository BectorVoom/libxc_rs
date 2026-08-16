//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1207/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1207<F: Float>(t1468: F, t17501: F, t27514: F, t5919: F, t17509: F, t94785: F, t28589: F, t4262: F, t17490: F, t27520: F, t27529: F, t28610: F) -> (F, F, F, F, F, F) {
    let t97713 = t1468 * t17501;
    let t97715 = t27514 * t5919;
    let t97717 = t94785 * t17509;
    let t97719 = t28589 * t4262;
    let t97721 = t27520 * t17490;
    let t97723 = t28610 * t27529;
    (t97713, t97715, t97717, t97719, t97721, t97723)
}
