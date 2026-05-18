//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1207/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1207<F: Float>(t2040: F, t28268: F, t4292: F, t572: F, t8453: F, t28265: F, t28280: F, t5795: F, t8614: F, t7334: F, t7944: F, t7324: F, t7953: F) -> (F, F, F, F, F, F, F) {
    let t127483 = t2040 * t28268;
    let t127489 = F::new(6.0) * t572 * t4292 * t8453;
    let t127490 = t2040 * t28265;
    let t127492 = t2040 * t28280;
    let t127495 = F::new(3.0) * t5795 * t8614;
    let t127496 = t7944 * t7334;
    let t127498 = t7324 * t7953;
    (t127483, t127489, t127490, t127492, t127495, t127496, t127498)
}
