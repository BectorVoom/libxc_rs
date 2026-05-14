//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 741/1141 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk741<F: Float>(t128: F, t12929: F, t10: F, t8144: F, t3637: F, t978: F, t102: F, t974: F, t8197: F, t120: F, t506: F, t12898: F, t5825: F, t127: F, t2893: F, t496: F, t5836: F, t8149: F, t8160: F, t8200: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t12930 = t128 * t12929;
    let t12931 = t10 * t12930;
    let t12934 = 0.97434166666666666666e0 * t8144;
    let t12937 = t978 * t3637;
    let t12946 = 0.1753815e2 * t102 * t974 * t3637;
    let t12947 = 0.19486833333333333333e1 * t8197;
    let t12949 = t120 * t12929;
    let t12951 = 0.2923025e1 * t102 * t12949;
    let t12952 = t506 * t12929;
    let t12955 = t5825 * t12898;
    let t12958 = t506 * t12898;
    let t12960 = 0.1753815e2 * t102 * t12958;
    let t12961 = -t496 * t12931 / 2.0 + t12934 - 0.293808e1 * t8149 - 0.146904e1 * t8160 + 9.0 / 2.0 * t496 * t10 * t12937 + 0.1762848e2 * t127 * t2893 * t3637 + t12946 - t12947 - 2.0 / 3.0 * t8200 + t5836 - t12951 - 0.146904e1 * t127 * t12952 - 0.293808e2 * t127 * t12955 - t12960;
    (t12930, t12931, t12934, t12937, t12946, t12947, t12949, t12951, t12952, t12955, t12958, t12960, t12961)
}
