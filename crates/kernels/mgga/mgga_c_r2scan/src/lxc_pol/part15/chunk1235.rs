//! MGGA_C_R2SCAN lxc pol — lxc_pol part 15 (v4rho3sigma_5) CSE chunk 1235/1253 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part15_v4rho3sigma_5_chunk1235<F: Float>(t40806: F, t1010: F, t37040: F, t11882: F, t19146: F, t37043: F, t37048: F, t37055: F, t37069: F, t40779: F, t40782: F, t40786: F, t40788: F, t40790: F, t40792: F, t40794: F, t40798: F, t40800: F, t40802: F, t40805: F, param_eta: F) -> F {
    let t40807 = F::new(4.0) / F::new(3.0) * t40806;
    let t40808 = t37040 * t1010;
    let t40812 = t19146 * param_eta * t11882;
    let t40814 = -F::new(11.0) / F::new(9.0) * t40779 + t40782 - F::new(4.0) / F::new(3.0) * t37048 + F::new(2.0) * t37055 - F::new(2.0) / F::new(3.0) * t37069 + t40786 + F::new(22.0) / F::new(9.0) * t40788 + t40790 / F::new(4.0) + t40792 / F::new(4.0) + t40794 / F::new(2.0) - t40798 + t40800 / F::new(4.0) - F::new(3.0) / F::new(4.0) * t40802 - t40805 - t40807 + F::new(11.0) / F::new(9.0) * t40808 - t37043 / F::new(3.0) - F::new(3.0) / F::new(2.0) * t40812;
    t40814
}
