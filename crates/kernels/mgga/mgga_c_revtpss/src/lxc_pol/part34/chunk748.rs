//! MGGA_C_REVTPSS lxc pol — lxc_pol part 34 (v4rho3sigma_9) CSE chunk 748/1341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part34_v4rho3sigma_9_chunk748<F: Float>(t7145: F, t7817: F, t1651: F, t1976: F, t1678: F, t1982: F, t1695: F, t7160: F, t1089: F, t1668: F, t7168: F, t1984: F, t359: F, t7810: F) -> (F, F, F, F, F, F, F, F) {
    let t7818 = t7145 * t7817;
    let t7821 = t1976 * t1651;
    let t7822 = t7145 * t7821;
    let t7825 = t1982 * t1678;
    let t7828 = t1976 * t1695;
    let t7829 = t7160 * t7828;
    let t7833 = t7168 * t1668 * t1089;
    let t7837 = t1984 * t359 * t7810;
    (t7818, t7821, t7822, t7825, t7828, t7829, t7833, t7837)
}
