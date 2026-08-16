//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 654/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk654<F: Float>(t539: F, t73: F, t241: F, t4000: F, t820: F, t550: F, t72: F, t245: F, t225: F, t3999: F, t213: F, t4086: F) -> (F, F, F, F, F, F) {
    let t5650 = t539 * t73;
    let t5671 = t820 * t4000 * t241;
    let t5672 = t550 * t72;
    let t5673 = t5672 * t245;
    let t5744 = t225 * t3999;
    let t5745 = t213 * t5744;
    let t5755 = t213 * t4086;
    (t5650, t5671, t5673, t5744, t5745, t5755)
}
