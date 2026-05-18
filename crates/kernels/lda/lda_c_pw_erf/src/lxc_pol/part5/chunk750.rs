//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 750/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk750<F: Float>(t6810: F, t6842: F, t582: F, t186: F, t211: F, t1960: F, t835: F, t549: F, t820: F, t184: F, t813: F, t4041: F, t5057: F, t5172: F, t5179: F, t5186: F, t5190: F, t5192: F, t5194: F, t5198: F, t5200: F, t6785: F, t6786: F, t6790: F, t6792: F) -> (F, F, F, F, F, F, F, F, F) {
    let t6843 = t6810 + t6842;
    let t6844 = t582 * t6843;
    let t6845 = t186 * t6844;
    let t6847 = F::new(2.0) / F::new(15.0) * t211 * t6845;
    let t6849 = F::new(4.0) / F::new(15.0) * t1960 * t835;
    let t6850 = t549 * t820;
    let t6851 = t6850 * t184;
    let t6853 = F::new(8.0) / F::new(15.0) * t6851 * t813;
    let t6854 = -t5057 - t5172 - t6785 + t5179 - t6786 + t6790 + t4041 + t6792 - t5186 + t5190 + t5192 + t5194 - t5198 + t5200 - t6847 - t6849 + t6853;
    (t6843, t6844, t6845, t6847, t6849, t6850, t6851, t6853, t6854)
}
