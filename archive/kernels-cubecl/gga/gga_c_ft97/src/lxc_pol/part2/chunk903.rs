//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 903/1007 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk903<F: Float>(t13746: F, t13753: F, t13728: F, t13732: F, t13736: F, t13743: F, t13750: F, t13759: F, t13981: F, t9872: F, t9876: F, t13780: F) -> (F, F) {
    let t13983 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t13746;
    let t13984 = F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t13753;
    let t13986 = F::cast_from(4.0_f64) * t13728 - F::cast_from(22.0_f64) / F::cast_from(9.0_f64) * t13732 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t13736 - t13981 + F::cast_from(2.0_f64) * t13743 - t13983 - t13750 + t13984 - t9872 - t9876 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t13759;
    let t13993 = F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t13780;
    (t13986, t13993)
}
