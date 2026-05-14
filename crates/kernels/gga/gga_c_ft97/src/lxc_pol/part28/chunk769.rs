//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 769/1041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk769<F: Float>(t32494: F, t925: F, t8217: F, t32515: F, t1909: F, t1901: F, t32487: F, t32508: F, t32510: F, t32587: F, t34663: F, t34667: F, t34671: F, t34674: F, t34678: F, t34682: F, t446: F) -> (F, F, F, F, F) {
    let t34685 = t32494 * t925;
    let t34686 = t8217 * t34685;
    let t34689 = t32515 * t925;
    let t34690 = t1909 * t34689;
    let t34693 = 2.0 / 3.0 * t446 * t34663 + 2.0 / 3.0 * t446 * t34667 - t32487 - t32508 + t32510 - 4.0 / 3.0 * t1901 * t34671 + 2.0 / 9.0 * t1901 * t34674 - 2.0 / 9.0 * t1901 * t34678 - 2.0 / 9.0 * t1901 * t34682 - 2.0 / 9.0 * t1901 * t34686 + t1901 * t34690 / 9.0 - t32587;
    (t34685, t34686, t34689, t34690, t34693)
}
