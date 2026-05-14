//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 327/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk327<F: Float>(t1634: F, t760: F, t755: F, t1657: F, t763: F) -> (F, F, F, F, F, F, F) {
    let t1959 = 0.17123333333333333333e-1 * t1634;
    let t1964 = t760 * t760;
    let t1965 = 1.0 / t1964;
    let t1966 = t755 * t1965;
    let t1968 = 0.516475e0 * t1634;
    let t1971 = 0.104195e0 * t1657;
    let t1974 = 1.0 / t763;
    (t1959, t1964, t1965, t1966, t1968, t1971, t1974)
}
