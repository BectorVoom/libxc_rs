//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 969/1133 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk969<F: Float>(t11745: F, t18331: F, t11387: F, t7204: F, t7557: F, t11483: F, t11749: F, t2787: F, t11994: F, t33696: F, t33258: F, t3781: F, t11356: F, t9563: F, t9934: F, t474: F, t8837: F) -> (F, F, F, F, F, F, F) {
    let t33728 = t18331 * t11745;
    let t33731 = t7204 * t11387 * t7557;
    let t33734 = t2787 * t11483 * t11749;
    let t33741 = t33696 * t11994;
    let t33743 = t33258 * t3781;
    let t33746 = t9563 * t11356 * t9934;
    let t33748 = t474 * t8837;
    (t33728, t33731, t33734, t33741, t33743, t33746, t33748)
}
