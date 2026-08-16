//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 963/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk963<F: Float>(t141052: F, t28603: F, t2691: F, t33940: F, t173: F, t33897: F, t7470: F, t28680: F, t28660: F, t33892: F, t28677: F, t28652: F) -> (F, F, F, F, F, F) {
    let t142712 = t28603 * t141052;
    let t142725 = t2691 * t33940;
    let t142736 = t7470 * t173 * t33897;
    let t142737 = t28680 * t142736;
    let t142739 = t28660 * t142736;
    let t142743 = t7470 * t173 * t33892;
    let t142744 = t28677 * t142743;
    let t142746 = t28652 * t142743;
    (t142712, t142725, t142737, t142739, t142744, t142746)
}
