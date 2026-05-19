//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 825/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk825<F: Float>(t41686: F, t20671: F, t31041: F, t34818: F, t34264: F, t7030: F, t10177: F, t10523: F, t544: F, t899: F, t913: F, t12957: F, t1441: F) -> (F, F, F, F, F) {
    let t41687 = F::cast_from(0.76685851907841499353e0_f64) * t41686;
    let t41689 = t31041 * t20671 * t34818;
    let t41690 = F::cast_from(0.17041300423964777634e0_f64) * t41689;
    let t41691 = t34264 * t7030;
    let t41692 = F::cast_from(0.29792074959875355558e-1_f64) * t41691;
    let t41696 = t544 * t10523 * t899 * t913 * t10177;
    let t41697 = F::cast_from(0.17875244975925213335e0_f64) * t41696;
    let t41698 = t1441 * t12957;
    (t41687, t41690, t41692, t41697, t41698)
}
