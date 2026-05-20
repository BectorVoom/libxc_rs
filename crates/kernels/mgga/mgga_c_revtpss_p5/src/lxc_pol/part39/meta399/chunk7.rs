//! MGGA_C_REVTPSS lxc pol — lxc_pol part 39 (v4rho3tau_2) CSE chunk 1456/1507 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part39_v4rho3tau_2_chunk1456<F: Float>(t17186: F, t17859: F, t17912: F, t17961: F, t1277: F, t1828: F, t3738: F, t13182: F, t3566: F, t488: F, t1276: F, t1774: F) -> (F, F, F, F) {
    let t17963 = t17186 + t17859 + t17912 + t17961;
    let t17964 = t1277 * t17963;
    let t17967 = t1828 * t3738;
    let t17968 = t13182 * t17967;
    let t17973 = t3566 * t488;
    let t17974 = t1276 * t1774;
    (t17964, t17968, t17973, t17974)
}
