//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 527/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk527<F: Float>(t28: F, t6677: F, t89: F, t526: F, t6615: F, t27: F, t5898: F, t5915: F, t6659: F, t6663: F, t6667: F, t6671: F, t6675: F) -> (F, F, F, F) {
    let t6678 = t28 * t6677;
    let t6679 = t89 * t6678;
    let t6681 = t526 * t6615;
    let t6683 = t89 * t27 * t6681;
    let t6685 = t6659 / 12.0 + t5898 + t6663 / 18.0 + t6667 / 3.0 - t6671 / 6.0 + t5915 + t6675 / 9.0 + 2.0 / 3.0 * t6679 - t6683 / 3.0;
    (t6679, t6681, t6683, t6685)
}
