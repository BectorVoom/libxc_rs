//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1156/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1156<F: Float>(t1923: F, t25146: F, t7348: F, t25150: F, t7349: F, t26169: F, t6954: F, t26204: F, t6977: F, t25117: F, t1927: F, t72: F, t843: F) -> (F, F, F, F, F, F) {
    let t95230 = t1923 * t7348 * t25146;
    let t95241 = t25150 * t7349;
    let t95243 = t6954 * t26169;
    let t95246 = t1923 * t26204 * t6977;
    let t95248 = t25117 * t7349;
    let t95253 = F::cast_from(1232.0_f64) / F::cast_from(81.0_f64) * t1923 * t843 * t72 * t1927;
    (t95230, t95241, t95243, t95246, t95248, t95253)
}
