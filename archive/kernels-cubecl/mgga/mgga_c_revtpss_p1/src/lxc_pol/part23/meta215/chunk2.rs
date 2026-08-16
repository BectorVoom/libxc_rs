//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1268/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1268<F: Float>(t2924: F, t6145: F, t2930: F, t4571: F, t6094: F, t6098: F, t6102: F, t1621: F, t954: F) -> (F, F, F, F) {
    let t6147 = F::cast_from(0.16081979498692535067e2_f64) * t2924 * t6145;
    let t6152 = t2930 + F::cast_from(0.11415555555555555555e-1_f64) * t4571 - F::cast_from(0.11415555555555555555e-1_f64) * t6094 + F::cast_from(0.34246666666666666666e-1_f64) * t6098 - F::cast_from(0.17123333333333333333e-1_f64) * t6102;
    let t6157 = t1621 * t1621;
    let t6158 = t6157 * t954;
    (t6147, t6152, t6157, t6158)
}
