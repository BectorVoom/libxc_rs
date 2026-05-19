//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1029/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1029<F: Float>(t36198: F, t2035: F, t31010: F, t35246: F, t30780: F, t35225: F, t1439: F, t1992: F, t1460: F, t30148: F, t7323: F, t142: F, t3706: F) -> (F, F, F, F, F, F, F, F) {
    let t36199 = F::cast_from(0.47172138434406228102e-2_f64) * t36198;
    let t36205 = t2035 * t31010 * t35246;
    let t36206 = F::new(0.183375e0) * t36205;
    let t36207 = t30780 * t35225;
    let t36208 = F::new(0.916875e-1) * t36207;
    let t36209 = t1992 * t1439;
    let t36210 = t30780 * t36209;
    let t36211 = F::new(0.916875e-1) * t36210;
    let t36213 = t30148 * t1460;
    let t36214 = t2035 * t7323 * t36213;
    let t36215 = F::new(0.916875e-1) * t36214;
    let t36222 = t142 * t3706;
    (t36199, t36206, t36208, t36209, t36211, t36213, t36215, t36222)
}
