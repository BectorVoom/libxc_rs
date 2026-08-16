//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 768/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk768<F: Float>(t12537: F, t1661: F, t587: F, t1010: F, t10843: F, t1017: F, t10365: F, t1885: F, t1820: F, t2615: F, t3527: F, t12345: F, t591: F) -> (F, F, F, F, F, F, F, F) {
    let t12538 = t1661 * t12537;
    let t12540 = F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t587 * t12538;
    let t12542 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t10843 * t1010;
    let t12543 = t10365 * t1017;
    let t12544 = t1885 * t12543;
    let t12546 = F::cast_from(4.0_f64) / F::cast_from(5.0_f64) * t1820 * t12544;
    let t12548 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t2615 * t3527;
    let t12549 = t591 * t12345;
    (t12538, t12540, t12542, t12543, t12544, t12546, t12548, t12549)
}
