//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 840/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk840<F: Float>(t33: F, t892: F, t4433: F, t18875: F, t25759: F, t1113: F, t1544: F, t4343: F, t27375: F, t11064: F) -> (F, F, F, F, F, F) {
    let t27763 = t892 * t33;
    let t27764 = t27763 * t4433;
    let t27770 = t25759 * t18875;
    let t27773 = t1113 * t1544;
    let t27777 = t33 * t4343;
    let t27793 = t25759 * t27375;
    let t27799 = t11064 * t33;
    (t27764, t27770, t27773, t27777, t27793, t27799)
}
