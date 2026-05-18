//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 869/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk869<F: Float>(t858: F, t8989: F, t3065: F, t343: F, t8827: F, t1114: F, t6671: F, t8840: F, t337: F, t2121: F, t6644: F, t3148: F, t6484: F) -> (F, F, F, F, F, F, F, F) {
    let t8990 = t858 * t8989;
    let t8991 = t3065 * t8990;
    let t8994 = t8827 * t343;
    let t8995 = t858 * t8994;
    let t8996 = t3065 * t8995;
    let t9016 = t1114 * t6671;
    let t9026 = t8840 * t343;
    let t9027 = t337 * t9026;
    let t9028 = t2121 * t9027;
    let t9035 = t1114 * t6644;
    let t9041 = F::new(7.0) / F::new(72.0) * t6484 * t3148;
    (t8991, t8994, t8996, t9016, t9026, t9028, t9035, t9041)
}
