//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 895/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk895<F: Float>(t161: F, t18049: F, t148: F, t163: F, t5985: F, t547: F, t5984: F, t1332: F, t147: F, t164: F, t1964: F, t762: F) -> (F, F, F, F, F, F, F) {
    let t18050 = t18049 * t161;
    let t18053 = F::cast_from(0.31505407223141117834e-1_f64) * t148 * t18050 * t163;
    let t18067 = F::cast_from(0.756129773355386828e0_f64) * t5985;
    let t18072 = F::cast_from(0.47461239486605618761e-3_f64) * t5984 * t547;
    let t18075 = t1332 * t147;
    let t18077 = F::cast_from(0.14238371845981685628e-2_f64) * t18075 * t164;
    let t18079 = F::cast_from(0.37806488667769341401e0_f64) * t762 * t1964;
    (t18050, t18053, t18067, t18072, t18075, t18077, t18079)
}
