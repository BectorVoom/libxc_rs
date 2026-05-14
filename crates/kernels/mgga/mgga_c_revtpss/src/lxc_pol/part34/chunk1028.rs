//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 1028/1196 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk1028<F: Float>(t3336: F, t7840: F, t33: F, t892: F, t11064: F, t1032: F, t1892: F, t1955: F) -> (F, F, F, F, F) {
    let t27712 = t7840 * t3336;
    let t27763 = t892 * t33;
    let t27799 = t11064 * t33;
    let t27836 = t1892 * t1032;
    let t27837 = t1955 * t27836;
    (t27712, t27763, t27799, t27836, t27837)
}
