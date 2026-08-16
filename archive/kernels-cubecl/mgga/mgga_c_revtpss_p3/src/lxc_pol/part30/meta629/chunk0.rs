//! MGGA_C_REVTPSS lxc pol — lxc_pol part 30 (v4rho3sigma_5) CSE chunk 2193/2270 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part30_v4rho3sigma_5_chunk2193<F: Float>(t1497: F, t2311: F, t77: F, t4241: F, t640: F, t13420: F, t84: F, t10298: F, t1470: F, t2242: F, t4181: F, t4187: F) -> (F, F, F, F, F, F) {
    let t101172 = t77 * t2311 * t1497;
    let t101176 = t77 * t640 * t4241;
    let t101182 = t77 * t84 * t13420;
    let t101187 = t10298 * t1470;
    let t101190 = t2242 * t4181;
    let t101193 = t2242 * t4187;
    (t101172, t101176, t101182, t101187, t101190, t101193)
}
