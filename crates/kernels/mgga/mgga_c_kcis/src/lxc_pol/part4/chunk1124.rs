//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 1124/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk1124<F: Float>(t3873: F, t5481: F, t1319: F, t3809: F, t5556: F, t11402: F, t1897: F, t3781: F, t16048: F, t16050: F, t11408: F, t11409: F, t11411: F, t11413: F, t11415: F, t16046: F, t16052: F, t16057: F, t16062: F, t16067: F, t16071: F, t16075: F, t16080: F, t16084: F, t16088: F) -> (F, F, F, F) {
    let t16162 = t3873 * t5481;
    let t16163 = t16162 * t1319;
    let t16165 = t5556 * t3809;
    let t16167 = t11402 * t1897;
    let t16168 = t16167 * t3781;
    let t16183 = 4.0 / 27.0 * t16048;
    let t16184 = 4.0 / 9.0 * t16050;
    let t16194 = -t11408 - 8.0 / 27.0 * t11409 + 2.0 / 27.0 * t11411 - 2.0 / 9.0 * t11413 + t11415 / 9.0 - 4.0 / 27.0 * t16046 + t16183 - t16184 - 22.0 / 9.0 * t16052 - 10.0 / 27.0 * t16057 + 4.0 / 3.0 * t16062 + 8.0 / 9.0 * t16067 - 2.0 / 9.0 * t16071 - 2.0 * t16075 - 8.0 / 3.0 * t16080 + 2.0 / 3.0 * t16084 + 2.0 / 3.0 * t16088;
    (t16163, t16165, t16168, t16194)
}
