//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1082/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1082<F: Float>(t1955: F, t6888: F, t1882: F, t1903: F, t543: F, t1868: F, t1907: F, t1501: F, t1518: F) -> (F, F, F, F) {
    let t30071 = t1955 * t6888;
    let t30105 = t1903 * t1882 * t543;
    let t30122 = t1868 * t1907;
    let t30138 = t1501 * t1518;
    (t30071, t30105, t30122, t30138)
}
