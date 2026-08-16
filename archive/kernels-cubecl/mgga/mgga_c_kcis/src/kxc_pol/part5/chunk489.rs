//! MGGA_C_KCIS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 489/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part5_v3rho3_2_chunk489<F: Float>(t2035: F, t573: F, t2001: F, t556: F, t572: F, t1533: F, t1928: F, t41: F) -> (F, F, F, F, F) {
    let t2036 = t2035 * t573;
    let t2038 = t556 * t2001;
    let t2039 = t572 * t2038;
    let t2040 = t1533 * t2039;
    let t2042 = t1928 * t41;
    (t2036, t2038, t2039, t2040, t2042)
}
