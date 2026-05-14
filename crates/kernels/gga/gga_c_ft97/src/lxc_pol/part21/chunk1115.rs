//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 1115/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk1115<F: Float>(t22914: F, t29594: F, t108: F, t29569: F, t100147: F, t102071: F, t1286: F, t1564: F, t22873: F, t22922: F, t25528: F, t25558: F, t25574: F, t25579: F, t25605: F, t26128: F, t28: F, t29734: F, t29736: F, t29741: F, t3051: F, t3103: F, t379: F, t4458: F, t5495: F, t5501: F, t5507: F, t5624: F, t6413: F, t93864: F, t984: F) -> (F,) {
    let t115175 = t22914 * t29594;
    let t115181 = t29569 * t108;
    let t115208 = 2.0 / 9.0 * t5501 * t102071 * t25605 - 4.0 / 27.0 * t93864 + t100147 - t115175 / 27.0 + t5501 * t1564 * t22922 * t4458 / 9.0 - t5501 * t1564 * t115181 * t379 / 18.0 + t29741 * t5624 / 6.0 - 2.0 / 3.0 * t1286 * t28 * t5507 * t984 * t3103 - t5495 * t29736 / 3.0 - t1286 * t28 * t22873 * t29734 / 3.0 - 2.0 / 3.0 * t1286 * t28 * t25528 * t26128 - t25558 * t25574 / 9.0 - 2.0 / 9.0 * t6413 * t3051 * t25579;
    (t115208,)
}
