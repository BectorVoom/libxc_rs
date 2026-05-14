//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1106/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1106<F: Float>(t1001: F, t9355: F, t982: F, t2689: F, t3174: F, t116: F, t3270: F, t210: F, t31825: F, t31827: F, t31829: F, t31832: F, t31835: F, t31838: F, t31840: F, t31842: F, t31844: F, t31846: F, t31849: F) -> (F, F, F, F, F, F, F) {
    let t31851 = t9355 * t1001;
    let t31852 = t982 * t31851;
    let t31854 = t2689 * t3174;
    let t31855 = t982 * t31854;
    let t31857 = t116 * t3270;
    let t31858 = t210 * t31857;
    let t31860 = t31825 / 8.0 - t31827 / 4.0 - t31829 / 2.0 + t31832 / 4.0 + t31835 / 2.0 - t31838 / 8.0 + 3.0 / 4.0 * t31840 - t31842 / 32.0 + t31844 / 16.0 + t31846 / 4.0 - t31849 / 16.0 - t31852 / 4.0 + t31855 / 32.0 - 5.0 / 8.0 * t31858;
    (t31851, t31852, t31854, t31855, t31857, t31858, t31860)
}
