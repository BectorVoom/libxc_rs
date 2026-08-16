//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1013/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1013<F: Float>(t2435: F, t4322: F, t1596: F, t2873: F, t1614: F, t2942: F, t1606: F, t2439: F, t1593: F) -> (F, F, F, F, F) {
    let t15063 = t2435 * t4322;
    let t15101 = t1596 * t2873;
    let t15104 = t1614 * t2942;
    let t15123 = t2439 * t1606;
    let t15189 = t2435 * t1593;
    (t15063, t15101, t15104, t15123, t15189)
}
