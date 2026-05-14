//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1105/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1105<F: Float>(t50235: F, t5855: F, t26857: F, t8392: F, t1882: F, t26890: F, t26906: F, t26911: F, t26943: F, t26961: F, t26899: F, t2101: F, t6685: F, t1391: F, t9132: F, t26992: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t106807 = t50235 * t5855;
    let t106830 = 2.0 / 27.0 * t8392 * t26857;
    let t106837 = 4.0 / 9.0 * t1882 * t26890;
    let t106840 = 4.0 / 9.0 * t1882 * t26906;
    let t106842 = 4.0 / 9.0 * t1882 * t26911;
    let t106844 = 4.0 / 9.0 * t1882 * t26943;
    let t106847 = 2.0 / 9.0 * t1882 * t26961;
    let t106871 = 2.0 / 9.0 * t1882 * t26899;
    let t106875 = t2101 * t6685;
    let t106894 = t9132 * t1391;
    let t106906 = 2.0 / 27.0 * t8392 * t26992;
    (t106807, t106830, t106837, t106840, t106842, t106844, t106847, t106871, t106875, t106894, t106906)
}
