//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 2914/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk2914<F: Float>(t11294: F, t23565: F, t19128: F, t4590: F, t52219: F, t6145: F, t23467: F, t41883: F, t23547: F, t2869: F, t11385: F, t15396: F, t6141: F, t934: F) -> (F, F, F, F, F, F) {
    let t77639 = F::new(6.0) * t11294 * t23565;
    let t77641 = F::new(3.0) * t4590 * t19128;
    let t77643 = F::cast_from(0.48245938496077605201e2_f64) * t52219 * t6145;
    let t77645 = F::cast_from(0.96491876992155210402e2_f64) * t41883 * t23467;
    let t77647 = F::new(1.0) * t2869 * t23547;
    let t77657 = F::cast_from(0.1551780387578202009e4_f64) * t11385 * t6141 * t15396 * t934;
    (t77639, t77641, t77643, t77645, t77647, t77657)
}
