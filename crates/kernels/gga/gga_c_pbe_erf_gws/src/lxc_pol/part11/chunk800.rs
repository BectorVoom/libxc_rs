//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 800/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk800<F: Float>(t128: F, t12929: F, t10: F, t8144: F, t3637: F, t978: F, t102: F, t974: F, t8197: F, t120: F, t506: F, t12898: F, t5825: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t12930 = t128 * t12929;
    let t12931 = t10 * t12930;
    let t12934 = F::cast_from(0.97434166666666666666e0_f64) * t8144;
    let t12937 = t978 * t3637;
    let t12946 = F::cast_from(0.1753815e2_f64) * t102 * t974 * t3637;
    let t12947 = F::cast_from(0.19486833333333333333e1_f64) * t8197;
    let t12949 = t120 * t12929;
    let t12951 = F::cast_from(0.2923025e1_f64) * t102 * t12949;
    let t12952 = t506 * t12929;
    let t12955 = t5825 * t12898;
    (t12930, t12931, t12934, t12937, t12946, t12947, t12949, t12951, t12952, t12955)
}
