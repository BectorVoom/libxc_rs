//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1063/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1063<F: Float>(t41463: F, t41466: F, t41469: F, t41474: F, t13019: F, t2103: F, t4673: F, t11807: F, t3277: F, t10057: F, t13045: F, t11004: F, t1445: F, t2087: F, t2530: F) -> (F, F, F, F, F, F, F, F) {
    let t44152 = F::cast_from(0.17875244975925213335e0_f64) * t41463;
    let t44154 = F::cast_from(0.29792074959875355558e-1_f64) * t41466;
    let t44155 = F::cast_from(0.29792074959875355558e-1_f64) * t41469;
    let t44156 = F::cast_from(0.25561950635947166451e1_f64) * t41474;
    let t44159 = t2103 * t4673 * t13019;
    let t44162 = F::cast_from(0.25025342966295298669e1_f64) * t3277 * t11807;
    let t44164 = F::cast_from(0.25025342966295298669e1_f64) * t10057 * t13045;
    let t44167 = t2087 * t1445 * t11004 * t2530;
    (t44152, t44154, t44155, t44156, t44159, t44162, t44164, t44167)
}
