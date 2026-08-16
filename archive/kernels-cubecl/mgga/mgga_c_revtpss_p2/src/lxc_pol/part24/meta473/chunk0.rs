//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1454/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1454<F: Float>(t2439: F, t2440: F, t6072: F, t15003: F, t51258: F, t6042: F, t786: F, t867: F, t14485: F, t14987: F, t2435: F, t6093: F) -> (F, F, F, F, F) {
    let t63050 = t2439 * t2440 * t6072;
    let t63058 = t51258 * t15003;
    let t63084 = t786 * t6042 * t867;
    let t63099 = t14987 * t14485;
    let t63453 = t2435 * t6093;
    (t63050, t63058, t63084, t63099, t63453)
}
