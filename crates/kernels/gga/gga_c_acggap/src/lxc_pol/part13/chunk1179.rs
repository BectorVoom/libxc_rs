//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1179/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1179<F: Float>(t1423: F, t7746: F, t31752: F, t31751: F, t36097: F, t36100: F, t36103: F, t36107: F, t36111: F, t36115: F, t36119: F, t36123: F, t36126: F, t36127: F, t36129: F, t36132: F, t36134: F, t36135: F, t36137: F) -> F {
    let t36139 = t7746 * t1423;
    let t36141 = F::new(0.26416397523267487738e-1) * t31752;
    let t36142 = t36097 + F::new(0.15724046144802076034e-3) * t36100 + F::new(0.10718504529517434243e-2) * t36103 + F::new(0.10718504529517434243e-2) * t36107 + F::new(0.53592522647587171215e-3) * t36111 - F::new(0.21437009059034868486e-3) * t36115 + F::new(0.41930789719472202757e-3) * t36119 - F::new(0.62896184579208304135e-3) * t36123 + t36126 + F::new(0.37737710747524982482e-2) * t36127 - F::new(0.21437009059034868486e-3) * t36129 - t36132 - t36134 - F::new(0.28582678745379824648e-3) * t36135 + F::new(0.19812298142450615803e-1) * t36137 - F::new(0.16006300097412701803e-1) * t36139 - t31751 - t36141;
    t36142
}
