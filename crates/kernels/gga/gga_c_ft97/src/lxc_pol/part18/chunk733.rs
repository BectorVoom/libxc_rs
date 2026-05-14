//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 733/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk733<F: Float>(t12590: F, t9049: F, t446: F, t12346: F, t12353: F, t12357: F, t12359: F, t12362: F, t12366: F, t12564: F, t12568: F, t12571: F, t12574: F, t12577: F, t12580: F, t12584: F, t12589: F, t8799: F, t8802: F, t9059: F, t9062: F, t9072: F) -> (F, F) {
    let t12591 = t9049 * t12590;
    let t12592 = t446 * t12591;
    let t12595 = -t12346 + t8799 / 54.0 + t8802 / 81.0 - t9059 / 27.0 + 2.0 / 3.0 * t12353 - t12357 + 11.0 / 27.0 * t12359 - 2.0 / 81.0 * t12362 - t9072 + t12366 - t12564 / 6.0 - t12568 / 9.0 - 2.0 / 27.0 * t12571 + t12574 / 9.0 + 4.0 / 9.0 * t12577 - t12580 / 27.0 + t12584 / 3.0 - t12589 + 2.0 / 27.0 * t12592 - t9062 / 27.0;
    (t12592, t12595)
}
