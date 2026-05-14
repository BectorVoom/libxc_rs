//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1046/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1046<F: Float>(t198: F, t1993: F, t11064: F, t30: F, t33: F, t892: F, t1032: F, t1892: F, t1955: F) -> (F, F, F, F, F, F) {
    let t27382 = t198 * t1993;
    let t27383 = t11064 * t30;
    let t27763 = t892 * t33;
    let t27799 = t11064 * t33;
    let t27836 = t1892 * t1032;
    let t27837 = t1955 * t27836;
    (t27382, t27383, t27763, t27799, t27836, t27837)
}
