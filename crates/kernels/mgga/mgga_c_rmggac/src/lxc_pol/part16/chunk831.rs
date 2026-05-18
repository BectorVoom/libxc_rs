//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 831/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk831<F: Float>(t2068: F, t41056: F, t305: F, t38674: F, t118: F, t25809: F, t39692: F, t5271: F, t6444: F, t9000: F, t25529: F, t27: F) -> (F, F, F, F, F, F) {
    let t41057 = t2068 * t41056;
    let t41114 = t305 * t38674;
    let t41116 = t118 * t25809;
    let t41120 = t5271 * t39692;
    let t41128 = t6444 * t9000;
    let t41130 = t25529 * t27;
    (t41057, t41114, t41116, t41120, t41128, t41130)
}
