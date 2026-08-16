//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 20 (v4rho3sigma_8) CSE chunk 985/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part20_v4rho3sigma_8_chunk985<F: Float>(t11132: F, t1621: F, t1620: F, t3390: F, t5109: F, t661: F, t639: F, t2615: F, t2689: F, t2556: F, t2562: F, t11108: F, t11109: F, t11114: F, t11118: F, t11120: F, t11122: F, t11124: F, t11128: F, t11130: F, t5562: F, t7968: F, t7970: F) -> (F, F, F, F, F, F) {
    let t11133 = t1621 * t11132;
    let t11135 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1620 * t11133;
    let t11136 = t5109 * t3390;
    let t11137 = t11136 * t661;
    let t11138 = t1621 * t11137;
    let t11140 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t639 * t11138;
    let t11142 = F::cast_from(8.0_f64) / F::cast_from(45.0_f64) * t2615 * t2689;
    let t11144 = F::cast_from(16.0_f64) / F::cast_from(45.0_f64) * t2615 * t2556;
    let t11146 = F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t2615 * t2562;
    let t11147 = t5562 - t11108 + t11109 + t11114 + t11118 + t11120 - t11122 + t11124 + t11128 + t11130 + t11135 - t11140 + t7968 + t7970 - t11142 - t11144 + t11146;
    (t11135, t11140, t11142, t11144, t11146, t11147)
}
