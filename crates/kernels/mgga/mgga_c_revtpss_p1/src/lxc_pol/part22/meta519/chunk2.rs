//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 2288/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk2288<F: Float>(t1132: F, t16926: F, t16708: F, t16717: F, t16722: F, t16735: F, t16740: F, t16744: F, t16908: F, t16927: F, t16929: F, t16931: F) -> (F, F) {
    let t16933 = t1132 * t16926;
    let t16940 = F::cast_from(0.36514074074074074075e-1_f64) * t16908 + F::new(0.3071625e0) * t16927 - t16929 + F::cast_from(0.13287407407407407408e0_f64) * t16708 + F::cast_from(0.36514074074074074074e-1_f64) * t16931 + F::new(0.1898925e1) * t16933 - F::cast_from(0.11958666666666666667e1_f64) * t16722 + F::cast_from(0.11958666666666666667e1_f64) * t16740 + F::cast_from(0.59793333333333333334e0_f64) * t16744 + F::new(0.17938e1) * t16735 + F::cast_from(0.33218518518518518518e0_f64) * t16717;
    (t16933, t16940)
}
