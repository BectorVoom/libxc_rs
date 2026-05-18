//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 627/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk627<F: Float>(t1873: F, t3957: F, t1353: F, t1872: F, t800: F, t124: F, t5591: F, t3938: F, t5674: F, t3936: F, t1399: F, t5673: F) -> (F, F, F, F, F, F, F) {
    let t5681 = t3957 * t1873;
    let t5686 = t800 * t1872 * t1353;
    let t5689 = t124 * t5591;
    let t5690 = t800 * t5689;
    let t5696 = t5674 * t3938;
    let t5697 = t3936 * t5696;
    let t5700 = t5674 * t1399;
    let t5701 = t5673 * t5700;
    (t5681, t5686, t5690, t5696, t5697, t5700, t5701)
}
