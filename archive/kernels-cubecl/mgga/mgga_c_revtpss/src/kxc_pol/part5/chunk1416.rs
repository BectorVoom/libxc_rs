//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1416/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1416<F: Float>(t4147: F, t6922: F, t566: F, t6816: F, t1448: F, t1868: F, t1353: F, t13664: F, t13682: F, t13683: F, t198: F, t22214: F, t22215: F, t22216: F, t22217: F, t22218: F, t22219: F, t4139: F, t4140: F, t5536: F, t5541: F, t5542: F, t5778: F, t6836: F, t9524: F, t9542: F, t9854: F, t9865: F, t9868: F) -> F {
    let t22483 = t6922 * t4147;
    let t22486 = t566 * t6816;
    let t22496 = t1868 * t1448;
    let t22504 = F::cast_from(6.0_f64) * t1353 * t198 * t566 * t6836 + F::cast_from(6.0_f64) * t1353 * t22486 * t5536 - t1448 * t22483 * t5541 - F::cast_from(6.0_f64) * t22496 * t4139 * t5542 + F::cast_from(3.0_f64) * t4139 * t4140 * t6816 - F::cast_from(2.0_f64) * t5541 * t5542 * t5778 - t13664 + t13682 + t13683 - t22214 + t22215 - t22216 - t22217 + t22218 + t22219 - t9524 + t9542 + t9854 + t9865 + t9868;
    t22504
}
