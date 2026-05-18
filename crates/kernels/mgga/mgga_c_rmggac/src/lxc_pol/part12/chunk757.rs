//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 757/1088 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk757<F: Float>(t35565: F, t2007: F, t7939: F, t1982: F, t7428: F, t7547: F, t7542: F, t321: F, t7817: F, t1550: F, t333: F, t903: F) -> (F, F, F, F, F, F, F, F) {
    let t35566 = F::new(0.24390119833260022651e-2) * t35565;
    let t35567 = t7939 * t2007;
    let t35577 = t7547 * t7428 * t1982;
    let t35580 = t7542 * t7428 * t1982;
    let t35583 = t7817 * t321;
    let t35584 = t1550 * t35583;
    let t35586 = t7817 * t333;
    let t35587 = t903 * t35586;
    (t35566, t35567, t35577, t35580, t35583, t35584, t35586, t35587)
}
