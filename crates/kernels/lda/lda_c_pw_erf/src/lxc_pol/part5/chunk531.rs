//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 531/1365 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk531<F: Float>(t1521: F, t1531: F, t1629: F, t1637: F, t1641: F, t2570: F, t2573: F, t2574: F, t2575: F, t2576: F, t2577: F, t2578: F) -> F {
    let t2666 = t2570 + t1629 + t1637 + t1641 - t1521 - t1531 - t2573 + t2574 - t2575 + t2576 + t2577 + t2578;
    t2666
}
