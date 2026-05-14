//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 653/1170 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk653<F: Float>(t550: F, t72: F, t245: F, t225: F, t3999: F, t213: F, t4086: F, t2242: F, t38: F, t644: F, t84: F, t77: F) -> (F, F, F, F, F, F) {
    let t5672 = t550 * t72;
    let t5673 = t5672 * t245;
    let t5744 = t225 * t3999;
    let t5745 = t213 * t5744;
    let t5755 = t213 * t4086;
    let t6954 = t2242 * t38;
    let t6959 = t84 * t644;
    let t6960 = t77 * t6959;
    (t5673, t5744, t5745, t5755, t6954, t6960)
}
