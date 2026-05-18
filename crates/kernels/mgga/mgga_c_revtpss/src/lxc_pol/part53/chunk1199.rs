//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1199/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1199<F: Float>(t33913: F, t7313: F, t196: F, t197: F, t28230: F, t2035: F, t32103: F, t4248: F, t27123: F, t8457: F, t27126: F, t32311: F, t7732: F) -> (F, F, F, F, F, F) {
    let t127314 = t33913 * t7313;
    let t127317 = t28230 * t196 * t197;
    let t127318 = t127317 * t2035;
    let t127324 = t4248 * t32103;
    let t127326 = t27123 * t8457;
    let t127328 = t27126 * t8457;
    let t127330 = t7732 * t32311;
    (t127314, t127318, t127324, t127326, t127328, t127330)
}
