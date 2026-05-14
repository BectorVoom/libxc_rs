//! MGGA_C_PKZB lxc pol — lxc_pol part 10 (v4rho4_2) CSE chunk 760/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part10_v4rho4_2_chunk760<F: Float>(t1923: F, t1928: F, t2730: F, t2772: F, t3517: F, t3529: F, t3533: F, t3537: F, t3539: F, t3544: F, t3548: F, t703: F) -> (F, F) {
    let t3577 = -0.17648625e1 * t3529 + 0.3529725e1 * t3533 + t1923 - 0.103295e1 * t2730 + 0.1549425e1 * t3517 + 0.31558125e0 * t3537 + 0.6311625e0 * t3539 + t1928 - 0.41678e0 * t2772 + 0.312585e0 * t3544 + 0.312585e0 * t3548;
    let t3578 = t3577 * t703;
    (t3577, t3578)
}
