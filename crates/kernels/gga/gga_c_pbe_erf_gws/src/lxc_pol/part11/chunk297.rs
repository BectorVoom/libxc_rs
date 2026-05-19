//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 11 (v4rho4_4) CSE chunk 297/1302 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part11_v4rho4_4_chunk297<F: Float>(t1022: F, t220: F, t186: F, t616: F, t626: F, t954: F, t625: F, t11: F, t624: F, t203: F, t184: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1023 = t220 * t1022;
    let t1024 = t186 * t1023;
    let t1026 = F::new(4.0) / F::new(15.0) * t616 * t1024;
    let t1027 = t626 * t954;
    let t1028 = t625 * t1027;
    let t1029 = t11 * t1028;
    let t1031 = t624 + F::cast_from(0.18891666666666666667e-2_f64) * t1029;
    let t1032 = t203 * t1031;
    let t1033 = t1032 * t184;
    (t1023, t1024, t1026, t1027, t1028, t1029, t1031, t1032, t1033)
}
