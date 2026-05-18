//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1050/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1050<F: Float>(t1981: F, t41799: F, t41800: F, t676: F, t236: F, t498: F, t3134: F, t8512: F, t1982: F, t7428: F, t8511: F, t16156: F, t9198: F) -> (F, F, F, F) {
    let t41803 = t41799 * t1981 * t676 * t41800;
    let t41805 = t236 * t498;
    let t41808 = t8512 * t1981 * t3134 * t41805;
    let t41811 = t8511 * t7428 * t1982;
    let t41812 = F::new(0.19863479950205658386e-4) * t41811;
    let t41813 = t16156 * t9198;
    (t41803, t41808, t41812, t41813)
}
