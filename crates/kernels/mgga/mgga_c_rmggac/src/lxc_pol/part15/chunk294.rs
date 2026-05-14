//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 294/963 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk294<F: Float>(t1004: F, t1011: F, t1014: F, t1017: F, t1425: F, t1535: F, t1813: F, t1814: F, t1815: F, t1816: F, t1817: F, t1819: F, t1835: F, t436: F, t948: F, t975: F, t982: F) -> (F,) {
    let t1838 = t948 - t975 + 0.186546e0 * t1425 * t1535 + t1813 - t1814 + t1815 - t1816 - t1817 + t982 - 0.31091e-1 * t1819 * t1004 + 0.93273e-1 * t436 * t1835 + t1011 + t1014 + t1017;
    (t1838,)
}
