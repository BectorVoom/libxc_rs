//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 895/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk895<F: Float>(t18124: F, t4440: F, t1607: F, t5713: F, t1610: F, t5477: F, t16082: F, t6159: F, t1369: F, t531: F, t617: F, t737: F, t110: F, t2105: F, t1599: F, t18093: F, t18096: F, t18100: F, t18105: F, t18110: F, t18116: F, t18121: F, t4439: F) -> (F, F) {
    let t18125 = t4440 * t18124;
    let t18128 = t5713 * t1607;
    let t18129 = t5477 * t1610;
    let t18130 = t18128 * t18129;
    let t18133 = t6159 * t16082;
    let t18137 = t1369 * t617 * t531;
    let t18138 = t737 * t18137;
    let t18141 = t110 * t2105;
    let t18142 = t1599 * t18141;
    let t18144 = -t18093 - t4439 * t18096 / 288.0 - t4439 * t18100 / 576.0 + t4439 * t18105 / 288.0 - t4439 * t18110 / 432.0 - t1599 * t18116 / 192.0 + t4439 * t18121 / 144.0 - t4439 * t18125 / 576.0 + t4439 * t18130 / 144.0 - t4439 * t18133 / 288.0 - t1599 * t18138 / 288.0 + t18142 / 864.0;
    (t18128, t18144)
}
