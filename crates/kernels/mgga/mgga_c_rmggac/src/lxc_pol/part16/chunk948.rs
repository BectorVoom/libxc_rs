//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 948/1012 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk948<F: Float>(t2211: F, t30453: F, t43157: F, t43158: F, t43169: F, t45811: F, t45813: F, t45818: F, t45822: F, t45825: F, t45827: F, t45830: F, t45832: F, t45836: F, t45844: F, t45846: F, t4985: F, t5055: F, t6473: F, t739: F, t9399: F, t9402: F, t9405: F) -> (F,) {
    let t48662 = 0.35922725105591425692e0 * t5055 * t9399 - 0.47896966807455234256e0 * t6473 * t9402 - 0.23948483403727617128e0 * t4985 * t9405 - 0.5107751987195740728e-4 * t45811 + 0.5107751987195740728e-4 * t45813 + 0.1702583995731913576e-4 * t45818 + 0.15323255961587222184e-3 * t45822 + 0.1702583995731913576e-4 * t45825 + 0.212822999466489197e-4 * t45827 - t43157 + t43158 + 0.13637330827122670865e0 * t45830 + 0.13637330827122670865e-1 * t45832 - 0.10215503974391481456e-3 * t45836 + t43169 - 0.1702583995731913576e-4 * t45844 - 0.85129199786595678799e-5 * t45846 + 0.11974241701863808564e0 * t739 * t2211 * t30453;
    (t48662,)
}
