//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 976/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk976<F: Float>(t547: F, t5984: F, t1597: F, t1964: F, t1332: F, t147: F, t164: F, t762: F, t1602: F, t5676: F, t1457: F, t5668: F) -> (F, F, F, F, F, F, F, F, F) {
    let t18072 = F::new(0.47461239486605618761e-3) * t5984 * t547;
    let t18073 = t1597 * t1964;
    let t18075 = t1332 * t147;
    let t18077 = F::new(0.14238371845981685628e-2) * t18075 * t164;
    let t18079 = F::new(0.37806488667769341401e0) * t762 * t1964;
    let t18080 = t1602 * t1964;
    let t18082 = t5676 * t164;
    let t18084 = t1457 * t547;
    let t18086 = t5668 * t164;
    (t18072, t18073, t18075, t18077, t18079, t18080, t18082, t18084, t18086)
}
