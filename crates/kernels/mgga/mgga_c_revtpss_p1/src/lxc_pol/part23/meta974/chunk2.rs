//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3312/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3312<F: Float>(t23059: F, t4147: F, t23087: F, t9593: F, t566: F, t6836: F, t198: F, t21969: F, t40076: F, t40079: F, t4139: F, t47152: F, t48327: F, t48330: F, t48332: F, t48334: F, t5532: F, t5591: F, t85993: F, t85994: F) -> (F, F, F) {
    let t86825 = t23059 * t4147;
    let t86828 = t23087 * t9593;
    let t86839 = t6836 * t566;
    let t86846 = F::cast_from(18.0_f64) * t198 * t5591 * t86839 + F::cast_from(9.0_f64) * t21969 * t4139 * t5532 + t40076 - t40079 + t47152 - t48327 - t48330 + t48332 - t48334 - t85993 + t85994;
    (t86825, t86828, t86846)
}
