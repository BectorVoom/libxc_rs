//! MGGA_C_REVTPSS lxc pol — lxc_pol part 26 (v4rho3sigma_1) CSE chunk 1164/1225 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part26_v4rho3sigma_1_chunk1164<F: Float>(t10260: F, t10415: F, t10416: F, t13207: F, t13216: F, t2055: F, t2089: F, t2322: F, t2372: F, t25082: F, t25188: F, t26154: F, t26218: F, t26383: F, t26405: F, t26412: F, t26679: F, t28167: F, t28196: F, t28286: F, t28658: F, t3813: F, t4254: F, t49630: F, t49654: F, t508: F, t569: F, t651: F, t7235: F, t7357: F, t7359: F, t7374: F, t7378: F, t7539: F, t9069: F, t95405: F, t95408: F, t9984: F) -> F {
    let t95446 = F::new(6.0) * t28196 * t28286 * t49654 - F::new(9.0) * t25082 * t26405 * t49630 + t95408 * t569 + F::new(18.0) * t7235 * t26412 + F::new(9.0) * t7235 * t26383 - F::new(2.0) * t651 * t508 * t95405 - F::new(6.0) * t10416 * t7378 - F::new(3.0) * t7357 * t3813 - F::new(3.0) * t25188 * t7539 - F::new(6.0) * t28658 * t2372 - F::new(6.0) * t7359 * t13216 - F::new(2.0) * t7359 * t10260 - F::new(6.0) * t10416 * t7374 - F::new(6.0) * t4254 * t26154 - F::new(6.0) * t2322 * t26218 - F::new(6.0) * t4254 * t26218 - F::new(2.0) * t651 * t13207 * t2055 + F::new(18.0) * t28167 * t9069 * t9984 + F::new(6.0) * t7235 * t26679 - t10415 * t2089;
    t95446
}
