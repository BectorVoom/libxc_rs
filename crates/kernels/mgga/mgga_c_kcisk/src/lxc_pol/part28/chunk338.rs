//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 338/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk338<F: Float>(t1908: F, t1957: F, t1634: F, t1638: F, t760: F) -> (F, F, F, F, F) {
    let t1958 = t1908 * t1957;
    let t1959 = 0.17123333333333333333e-1 * t1634;
    let t1961 = -t1959 - 0.17123333333333333333e-1 * t1638;
    let t1964 = t760 * t760;
    let t1965 = 1.0 / t1964;
    (t1958, t1959, t1961, t1964, t1965)
}
