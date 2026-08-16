//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1154/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1154<F: Float>(t1364: F, t27968: F, t2022: F, t3999: F, t212: F, t7910: F, t1358: F, t689: F, t7925: F, t25904: F, t25899: F, t1513: F, t25823: F) -> (F, F, F, F, F, F, F, F, F) {
    let t27969 = t27968 * t1364;
    let t27980 = t3999 * t2022;
    let t27985 = t212 * t7910;
    let t27986 = t27985 * t1358;
    let t27987 = t689 * t27986;
    let t27989 = t7925 * t689;
    let t27990 = t25904 * t27989;
    let t27992 = t25899 * t27989;
    let t28034 = t25823 * t1513;
    (t27969, t27980, t27985, t27986, t27987, t27989, t27990, t27992, t28034)
}
