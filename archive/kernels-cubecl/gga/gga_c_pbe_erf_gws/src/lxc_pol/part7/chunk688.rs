//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 688/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk688<F: Float>(t5536: F, t587: F, t1759: F, t562: F, t2559: F, t1820: F, t1661: F, t597: F, t610: F, t1802: F, t590: F, t1804: F, t418: F, t572: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t5538 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t587 * t5536;
    let t5539 = t1759 * t562;
    let t5540 = t2559 * t5539;
    let t5542 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t1820 * t5540;
    let t5543 = t1661 * t597;
    let t5544 = t1759 * t610;
    let t5545 = t5543 * t5544;
    let t5547 = F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t587 * t5545;
    let t5548 = t590 * t1802;
    let t5550 = t1804 * t572 * t418;
    (t5538, t5539, t5540, t5542, t5543, t5544, t5545, t5547, t5548, t5550)
}
