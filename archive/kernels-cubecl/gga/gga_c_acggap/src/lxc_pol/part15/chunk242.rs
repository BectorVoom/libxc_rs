//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 242/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk242<F: Float>(t164: F, t980: F, t177: F, t38: F, t8: F, t121: F, t126: F, t147: F, t174: F, t879: F, t386: F, t387: F) -> (F, F, F, F, F, F, F) {
    let t981 = t980 * t164;
    let t983 = F::cast_from(0.21437009059034868486e-3_f64) * t981 * t177;
    let t985 = F::cast_from(1.0_f64) / t8 / t38;
    let t986 = t121 * t985;
    let t987 = t986 * t126;
    let t989 = F::cast_from(35.0_f64) / F::cast_from(432.0_f64) * t987 * t147;
    let t991 = t174 * t879;
    let t993 = t386 * t387 * t991;
    (t983, t985, t986, t987, t989, t991, t993)
}
