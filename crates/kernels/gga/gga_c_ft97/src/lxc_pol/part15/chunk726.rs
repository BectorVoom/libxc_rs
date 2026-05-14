//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 726/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk726<F: Float>(t1175: F, t5053: F, t729: F, t13927: F, t5064: F, t242: F, t21399: F, t265: F, t14224: F, t18593: F, t1901: F, t21641: F, t21647: F, t21652: F, t21657: F, t21661: F, t21665: F, t21669: F, t21674: F, t21678: F, t21682: F, t446: F) -> (F, F, F, F, F) {
    let t21686 = t729 * t1175 * t5053;
    let t21688 = t13927 * t5064;
    let t21689 = t242 * t21688;
    let t21693 = t729 * t265 * t21399;
    let t21696 = -t446 * t21641 / 3.0 + 2.0 / 3.0 * t18593 + t1901 * t21647 / 3.0 + t1901 * t21652 / 3.0 + 2.0 / 3.0 * t1901 * t21657 - 2.0 / 9.0 * t1901 * t21661 + t1901 * t21665 / 3.0 + t1901 * t21669 / 3.0 + 2.0 / 9.0 * t1901 * t21674 + 2.0 * t446 * t21678 - t446 * t21682 - 4.0 / 27.0 * t14224 - t446 * t21686 + 2.0 * t446 * t21689 - t446 * t21693 / 3.0;
    (t21686, t21688, t21689, t21693, t21696)
}
