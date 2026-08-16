//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 1004/1292 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk1004<F: Float>(t28875: F, t28887: F, t545: F, t2028: F, t689: F, t8099: F, t25904: F, t25899: F, t213: F, t8085: F, t1904: F, t7492: F) -> (F, F, F, F, F, F) {
    let t28888 = t28875 + t28887;
    let t28889 = t545 * t28888;
    let t28890 = t2028 * t28889;
    let t28894 = t8099 * t689;
    let t28895 = t25904 * t28894;
    let t28897 = t25899 * t28894;
    let t28899 = t213 * t8085;
    let t28902 = t7492 * t1904;
    (t28888, t28890, t28895, t28897, t28899, t28902)
}
