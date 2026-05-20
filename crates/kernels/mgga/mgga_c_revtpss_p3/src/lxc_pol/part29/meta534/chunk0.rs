//! MGGA_C_REVTPSS lxc pol — lxc_pol part 29 (v4rho3sigma_4) CSE chunk 1866/2049 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part29_v4rho3sigma_4_chunk1866<F: Float>(t2435: F, t26447: F, t26485: F, t93342: F, t10509: F, t26481: F, t25387: F, t11015: F, t7388: F, t212: F, t26473: F, t689: F, t780: F) -> (F, F, F, F, F, F) {
    let t95620 = t2435 * t26447;
    let t95624 = t93342 * t26485;
    let t95628 = t26481 * t10509;
    let t95629 = t25387 * t95628;
    let t95632 = F::cast_from(0.30356481678079769392e-1_f64) * t7388 * t11015;
    let t95635 = t689 * t212 * t26473 * t780;
    (t95620, t95624, t95628, t95629, t95632, t95635)
}
