//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 957/1093 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk957<F: Float>(t25227: F, t4353: F, t2661: F, t1565: F, t25222: F, t241: F, t25260: F, t820: F, t1955: F, t7057: F, t11064: F, t30: F, t33: F, t892: F, t1032: F, t1892: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27253 = t25227 * t4353;
    let t27254 = t2661 * t27253;
    let t27256 = t25222 * t1565;
    let t27261 = t820 * t25260 * t241;
    let t27353 = t1955 * t7057;
    let t27383 = t11064 * t30;
    let t27763 = t892 * t33;
    let t27799 = t11064 * t33;
    let t27836 = t1892 * t1032;
    (t27253, t27254, t27256, t27261, t27353, t27383, t27763, t27799, t27836)
}
