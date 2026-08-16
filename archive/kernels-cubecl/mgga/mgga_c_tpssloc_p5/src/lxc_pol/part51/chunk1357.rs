//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1357/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1357<F: Float>(t19456: F, t8533: F, t31772: F, t4028: F, t12725: F, t33234: F, t6525: F, t1388: F, t22574: F, t26558: F, t33357: F, t33610: F, t6876: F) -> (F, F, F, F, F, F) {
    let t120924 = F::cast_from(2.0_f64) * t19456 * t8533;
    let t120926 = F::cast_from(2.0_f64) * t4028 * t31772;
    let t120928 = F::cast_from(2.0_f64) * t12725 * t8533;
    let t120930 = F::cast_from(2.0_f64) * t33234 * t6525;
    let t120940 = F::cast_from(6.0_f64) * t22574 * t26558 * t33357 * t1388;
    let t120941 = t6876 * t33610;
    (t120924, t120926, t120928, t120930, t120940, t120941)
}
