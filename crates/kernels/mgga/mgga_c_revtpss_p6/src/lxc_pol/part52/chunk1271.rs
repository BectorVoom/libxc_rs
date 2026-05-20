//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1271/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1271<F: Float>(t34298: F, t98588: F, t2014: F, t28926: F, t8717: F, t128557: F, t128560: F, t128562: F, t128572: F, t128574: F, t128577: F, t128867: F, t26399: F, t27145: F, t28658: F, t28939: F, t33913: F, t7359: F, t7539: F, t7746: F, t8568: F) -> F {
    let t128869 = F::new(2.0) * t98588 * t34298;
    let t128871 = t2014 * t28926 * t8717;
    let t128872 = -F::new(2.0) * t26399 * t7746 - F::new(2.0) * t27145 * t7359 - F::new(2.0) * t28658 * t7746 + F::new(3.0) * t28939 * t8568 - t33913 * t7539 - t128557 - t128560 - t128562 + t128572 - t128574 + t128577 + t128867 + t128869 - t128871;
    t128872
}
