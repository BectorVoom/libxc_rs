//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 873/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk873<F: Float>(t30347: F, t30811: F, t4273: F, t129: F, t507: F, t7585: F, t7587: F, t30546: F, t8477: F, t1967: F, t8561: F, t30543: F, t8515: F, t30398: F, t30416: F, t10146: F, t420: F, t576: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t34339 = 0.42874018118069736972e-3 * t30347;
    let t34340 = t30811 * t4273;
    let t34345 = t129 * t507;
    let t34347 = t7585 * t34345 * t7587;
    let t34349 = t30546 * t8477;
    let t34351 = t1967 * t8561;
    let t34361 = t30543 * t8515;
    let t34364 = 35.0 / 216.0 * t30398;
    let t34366 = 0.25158473831683321654e-2 * t30416;
    let t34368 = t576 * t420 * t10146;
    (t34339, t34340, t34345, t34347, t34349, t34351, t34361, t34364, t34366, t34368)
}
