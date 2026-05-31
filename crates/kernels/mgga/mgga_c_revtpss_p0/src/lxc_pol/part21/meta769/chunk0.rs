//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2723/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2723<F: Float>(t2609: F, t4395: F, t14341: F, t2398: F, t40145: F, t11084: F, t15078: F, t40141: F, t4433: F, t4541: F, t50080: F, t50085: F, t50091: F, t50093: F, t50095: F, t50096: F) -> (F, F, F, F) {
    let t50097 = t4395 * t2609;
    let t50098 = F::cast_from(3.0_f64) * t50097;
    let t50099 = t2398 * t14341;
    let t50100 = F::cast_from(24.0_f64) * t50099;
    let t50101 = F::cast_from(12.0_f64) * t40145;
    let t50102 = -F::cast_from(18.0_f64) * t11084 * t4433 * t4541 + F::cast_from(18.0_f64) * t15078 * t50080 + t40141 + t50085 + t50091 + t50093 + t50095 + t50096 + t50098 + t50100 + t50101;
    (t50098, t50100, t50101, t50102)
}
