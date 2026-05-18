//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 1018/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk1018<F: Float>(t41466: F, t41469: F, t41474: F, t41477: F, t13019: F, t2103: F, t4673: F, t11807: F, t3277: F, t10057: F, t13045: F, t11004: F, t1445: F, t2087: F, t2530: F) -> (F, F, F, F, F, F, F, F) {
    let t44154 = F::new(0.29792074959875355558e-1) * t41466;
    let t44155 = F::new(0.29792074959875355558e-1) * t41469;
    let t44156 = F::new(0.25561950635947166451e1) * t41474;
    let t44157 = F::new(0.12780975317973583225e0) * t41477;
    let t44159 = t2103 * t4673 * t13019;
    let t44162 = F::new(0.25025342966295298669e1) * t3277 * t11807;
    let t44164 = F::new(0.25025342966295298669e1) * t10057 * t13045;
    let t44167 = t2087 * t1445 * t11004 * t2530;
    (t44154, t44155, t44156, t44157, t44159, t44162, t44164, t44167)
}
