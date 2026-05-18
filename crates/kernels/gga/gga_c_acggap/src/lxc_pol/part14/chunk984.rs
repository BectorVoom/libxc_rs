//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 984/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk984<F: Float>(t1980: F, t3201: F, t7458: F, t8569: F, t1988: F, t8549: F, t1095: F, t4806: F, t7476: F, t7799: F, t8555: F, t1530: F, t31056: F) -> (F, F, F, F, F) {
    let t34783 = t1980 * t7458 * t3201 * t8569;
    let t34794 = t1988 * t8549;
    let t34795 = F::new(0.15724046144802076034e-2) * t34794;
    let t34802 = t1980 * t7476 * t1095 * t4806;
    let t34803 = F::new(0.10482697429868050689e-2) * t34802;
    let t34804 = t7799 * t8555;
    let t34823 = t1530 * t31056;
    (t34783, t34795, t34803, t34804, t34823)
}
