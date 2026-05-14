//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 1034/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk1034<F: Float>(t4738: F, t6999: F, t6981: F, t17396: F, t17398: F, t17413: F, t17417: F, t184: F, t1958: F, t221: F, t2400: F, t21530: F, t21535: F, t21540: F, t21542: F, t21544: F, t21549: F) -> (F, F, F, F, F, F, F, F) {
    let t21551 = 4.0 / 5.0 * t4738 * t6999;
    let t21553 = 4.0 / 5.0 * t4738 * t6981;
    let t21554 = 32.0 / 45.0 * t17396;
    let t21555 = 64.0 / 45.0 * t17398;
    let t21556 = 4.0 / 15.0 * t17413;
    let t21557 = 16.0 / 15.0 * t17417;
    let t21561 = 4.0 / 5.0 * t2400 * t1958 * t184 * t221;
    let t21562 = t21530 + t21535 - t21540 + t21542 - t21544 - t21549 - t21551 - t21553 - t21554 - t21555 - t21556 + t21557 + t21561;
    (t21551, t21553, t21554, t21555, t21556, t21557, t21561, t21562)
}
