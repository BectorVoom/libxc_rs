//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1474/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1474<F: Float>(t2874: F, t41510: F, t935: F, t2866: F, t2873: F, t2876: F, t11298: F, t910: F, t11301: F, t11385: F, t2926: F, t41500: F) -> (F, F, F, F) {
    let t41879 = F::cast_from(6.0_f64) * t2874 * t41510 * t935;
    let t41880 = t2866 * t2873;
    let t41882 = F::cast_from(12.0_f64) * t41880 * t2876;
    let t41883 = t910 * t11298;
    let t41885 = F::cast_from(0.3859675079686208416e3_f64) * t41883 * t11301;
    let t41888 = F::cast_from(0.57895126195293126241e3_f64) * t11385 * t41500 * t2926;
    (t41879, t41882, t41885, t41888)
}
