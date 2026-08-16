//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2691/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2691<F: Float>(t5600: F, t9292: F, t1893: F, t4075: F, t786: F, t9682: F, t10115: F, t1894: F, t14094: F, t2435: F, t1358: F, t2439: F, t5710: F, t785: F) -> (F, F, F, F, F) {
    let t49468 = t9292 * t5600;
    let t49471 = t786 * t1893 * t4075;
    let t49472 = t49471 * t9682;
    let t49474 = t10115 * t1894;
    let t49476 = t2435 * t14094;
    let t49477 = F::cast_from(0.21951497276451705329e-1_f64) * t49476;
    let t49480 = t2439 * t785 * t5710 * t1358;
    (t49468, t49472, t49474, t49477, t49480)
}
