//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1014/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1014<F: Float>(t2133: F, t3039: F, t2138: F, t1114: F, t6187: F, t6543: F, t2195: F, t3131: F, t3139: F, t2168: F, t6566: F, t6570: F) -> (F, F, F, F, F, F, F, F) {
    let t9108 = t3039 * t2133;
    let t9110 = t9108 * t2138 / F::cast_from(48.0_f64);
    let t9111 = t1114 * t6187;
    let t9113 = t9111 * t2138 / F::cast_from(48.0_f64);
    let t9114 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t6543;
    let t9116 = t3139 * t3131 * t2195;
    let t9118 = t2168 * t9116 / F::cast_from(96.0_f64);
    let t9119 = t1114 * t6566;
    let t9121 = t9119 * t6570 / F::cast_from(48.0_f64);
    (t9108, t9110, t9111, t9113, t9114, t9116, t9118, t9121)
}
