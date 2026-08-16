//! MGGA_C_REVTPSS lxc pol — lxc_pol part 56 (v4rho2sigma2_11) CSE chunk 1182/1203 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part56_v4rho2sigma2_11_chunk1182<F: Float>(t124577: F, t33477: F, t131395: F, t33501: F, t127: F, t33509: F, t34899: F, t371: F, t131394: F, t8938: F, t8939: F, t33495: F, t34918: F) -> (F, F, F, F, F) {
    let t131608 = t33477 * t124577;
    let t131611 = t33501 * t131395;
    let t131616 = t33509 * t371 * t127 * t34899;
    let t131620 = t8938 * t8939 * t131394;
    let t131629 = t33495 * t371 * t127 * t34918;
    (t131608, t131611, t131616, t131620, t131629)
}
