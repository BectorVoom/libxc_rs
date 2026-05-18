//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 615/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk615<F: Float>(t5875: F, t5909: F, t5908: F, t4293: F, t5671: F, t4292: F, t5880: F, t4261: F, t4260: F, t1552: F, t2051: F, t2055: F, t4281: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5910 = t5909 * t5875;
    let t5911 = t5908 * t5910;
    let t5913 = t4293 * t5671;
    let t5914 = t4292 * t5913;
    let t5916 = t4293 * t5880;
    let t5917 = t4292 * t5916;
    let t5919 = t4261 * t5671;
    let t5920 = t4260 * t5919;
    let t5922 = t2051 * t1552;
    let t5924 = t4281 * t2055;
    (t5910, t5911, t5913, t5914, t5916, t5917, t5919, t5920, t5922, t5924)
}
