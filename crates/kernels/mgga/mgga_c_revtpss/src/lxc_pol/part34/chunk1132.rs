//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1132/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1132<F: Float>(t27418: F, t994: F, t27638: F, t3143: F, t1983: F, t1647: F, t1976: F, t3336: F, t7840: F, t33: F, t892: F, t11064: F) -> (F, F, F, F, F, F, F) {
    let t27661 = t994 * t27418;
    let t27668 = t27638 * t3143;
    let t27669 = t1983 * t27668;
    let t27699 = t1647 * t1976;
    let t27712 = t7840 * t3336;
    let t27763 = t892 * t33;
    let t27799 = t11064 * t33;
    (t27661, t27668, t27669, t27699, t27712, t27763, t27799)
}
