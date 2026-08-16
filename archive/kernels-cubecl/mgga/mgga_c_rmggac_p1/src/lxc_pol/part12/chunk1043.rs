//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 1043/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk1043<F: Float>(t1587: F, t236: F, t3351: F, t498: F, t7248: F, t26157: F, t5223: F, t645: F, t1635: F, t2064: F, t4044: F, t1550: F, t27102: F, t7577: F) -> (F, F, F, F) {
    let t41706 = t3351 * t7248 * t236 * t1587 * t498;
    let t41713 = t26157 * t645 * t5223;
    let t41716 = t4044 * t2064 * t1635;
    let t41717 = F::cast_from(0.95793933614910468512e0_f64) * t41716;
    let t41719 = t1550 * t7577 * t27102;
    (t41706, t41713, t41717, t41719)
}
