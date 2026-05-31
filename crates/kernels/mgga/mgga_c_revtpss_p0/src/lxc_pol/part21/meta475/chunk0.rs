//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2041/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2041<F: Float>(t2: F, t895: F, t580: F, t265: F, t22: F, t4567: F, t1610: F, t2875: F, t2924: F, t1596: F, t2873: F, t2876: F) -> (F, F, F, F, F, F, F, F) {
    let t15091 = t895 * t2;
    let t15093 = F::cast_from(2.0_f64) * t15091 * t580;
    let t15094 = t265 * t580;
    let t15096 = F::cast_from(3.0_f64) * t4567 * t22;
    let t15098 = t1610 * t2875;
    let t15100 = F::cast_from(6.0_f64) * t2924 * t15098;
    let t15101 = t1596 * t2873;
    let t15103 = F::cast_from(2.0_f64) * t15101 * t2876;
    (t15091, t15093, t15094, t15096, t15098, t15100, t15101, t15103)
}
