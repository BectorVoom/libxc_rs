//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1604/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1604<F: Float>(t3362: F, t3603: F, t2251: F, t12773: F, t12784: F, t13061: F, t44173: F, t10356: F, t1214: F, t12772: F, t12835: F, t3625: F) -> (F, F, F, F, F) {
    let t44190 = t3603 * t3362;
    let t44191 = t44190 * t2251;
    let t44200 = t12784 * t12773;
    let t44202 = t44173 * t13061;
    let t44205 = t10356 * t1214;
    let t44215 = t3625 * t12772 * t12835;
    (t44191, t44200, t44202, t44205, t44215)
}
