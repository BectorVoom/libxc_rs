//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 1186/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk1186<F: Float>(t17946: F, t764: F, t1693: F, t238: F, t2149: F, t2153: F, t5547: F, t2157: F, t64: F, t234: F, t339: F, t2165: F) -> (F, F, F, F, F, F) {
    let t17947 = t17946 * t764;
    let t17948 = F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t17947;
    let t17949 = t1693 * t238;
    let t17950 = t17949 * t2149;
    let t17952 = t5547 * t2153;
    let t17954 = t2157 * t64;
    let t17956 = t339 * t17954 * t234;
    let t17957 = t17956 * t2165;
    (t17947, t17948, t17950, t17952, t17954, t17957)
}
