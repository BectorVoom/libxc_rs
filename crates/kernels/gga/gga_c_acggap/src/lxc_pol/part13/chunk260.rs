//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 260/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk260<F: Float>(t164: F, t980: F, t177: F, t38: F, t8: F, t121: F, t126: F, t147: F, t165: F, t335: F, t397: F, t932: F, t936: F, t942: F, t947: F, t953: F, t957: F, t962: F, t968: F, t976: F, t979: F) -> (F, F, F, F, F, F) {
    let t981 = t980 * t164;
    let t983 = F::cast_from(0.21437009059034868486e-3_f64) * t981 * t177;
    let t985 = F::new(1.0) / t8 / t38;
    let t986 = t121 * t985;
    let t987 = t986 * t126;
    let t989 = F::new(35.0) / F::new(432.0) * t987 * t147;
    let t990 = -F::cast_from(0.21437009059034868486e-3_f64) * t397 * t932 - F::cast_from(0.42874018118069736972e-3_f64) * t936 + F::cast_from(0.42874018118069736972e-3_f64) * t942 * t947 + F::cast_from(0.20007875121765877254e-2_f64) * t953 - F::cast_from(0.21437009059034868486e-3_f64) * t397 * t957 + t335 * t962 / F::new(24.0) + F::cast_from(0.42874018118069736972e-3_f64) * t165 * t968 + t976 - t979 + t983 + t989;
    (t983, t985, t986, t987, t989, t990)
}
