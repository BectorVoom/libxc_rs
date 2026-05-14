//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 737/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk737<F: Float>(t1217: F, t21930: F, t5206: F, t5304: F, t1091: F, t5225: F, t10248: F, t446: F, t1212: F, t4969: F, t2665: F, t10270: F, t21181: F, t89: F, t9716: F, t10398: F, t14715: F, t14895: F, t19246: F, t19249: F, t19298: F, t19301: F, t19304: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t21931 = t21930 * t1217;
    let t21933 = t5206 * t5304;
    let t21945 = t1091 * t5225;
    let t21946 = t10248 * t21945;
    let t21947 = t446 * t21946;
    let t21949 = t4969 * t1212;
    let t21950 = t2665 * t21949;
    let t21951 = t446 * t21950;
    let t21953 = t10270 * t21181;
    let t21955 = t89 * t9716 * t21953;
    let t21957 = t19246 / 6.0 - t19249 / 3.0 + t19298 / 18.0 - t19301 / 9.0 + t19304 / 27.0 - 2.0 / 9.0 * t14895 - 2.0 / 27.0 * t14715 - t21947 / 3.0 - t21951 / 3.0 - t10398 - 5.0 / 81.0 * t21955;
    (t21931, t21933, t21945, t21946, t21947, t21949, t21950, t21951, t21953, t21955, t21957)
}
