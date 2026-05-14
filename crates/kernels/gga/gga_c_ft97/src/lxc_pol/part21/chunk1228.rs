//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1228/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1228<F: Float>(t104150: F, t104151: F, t104157: F, t1058: F, t1570: F, t16150: F, t16169: F, t1642: F, t16950: F, t17006: F, t24080: F, t24081: F, t26801: F, t26809: F, t26817: F, t27420: F, t27421: F, t27422: F, t27426: F, t27427: F, t27428: F, t30117: F, t30122: F, t3188: F, t378: F, t379: F, t5772: F, t6616: F) -> (F,) {
    let t118591 = 4.0 / 9.0 * t26809 * t27420 * t27421 * t16169 - 5.0 / 81.0 * t5772 * t104150 * t104151 * t16150 - 4.0 / 27.0 * t26809 * t27426 * t27427 * t16169 + 2.0 / 9.0 * t5772 * t27420 * t1058 * t1570 * t3188 - t5772 * t27420 * t27427 * t16150 / 3.0 - 2.0 / 9.0 * t5772 * t24080 * t27421 * t16950 + 2.0 / 9.0 * t5772 * t378 * t6616 * t27422 - 2.0 / 27.0 * t5772 * t1642 * t6616 * t27428 + 2.0 / 9.0 * t26817 * t26801 + 2.0 / 9.0 * t5772 * t24080 * t30117 * t379 + t5772 * t24080 * t30122 * t379 / 9.0 + t5772 * t24080 * t24081 * t17006 / 9.0 - t104157;
    (t118591,)
}
