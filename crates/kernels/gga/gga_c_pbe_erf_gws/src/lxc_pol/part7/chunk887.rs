//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 887/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk887<F: Float>(t163: F, t169: F, t366: F, t684: F, t5985: F, t413: F, t535: F, t164: F, t547: F, t5984: F, t1597: F, t1964: F, t1332: F, t147: F, t762: F, t1602: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t18065 = t169 * t366 * t684 * t163;
    let t18067 = 0.756129773355386828e0 * t5985;
    let t18068 = t413 * t535;
    let t18069 = t18068 * t164;
    let t18072 = 0.47461239486605618761e-3 * t5984 * t547;
    let t18073 = t1597 * t1964;
    let t18075 = t1332 * t147;
    let t18077 = 0.14238371845981685628e-2 * t18075 * t164;
    let t18079 = 0.37806488667769341401e0 * t762 * t1964;
    let t18080 = t1602 * t1964;
    (t18065, t18067, t18068, t18069, t18072, t18073, t18075, t18077, t18079, t18080)
}
