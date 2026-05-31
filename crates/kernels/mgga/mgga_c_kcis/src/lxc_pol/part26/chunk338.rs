//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 338/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk338<F: Float>(t1995: F, t542: F, t1102: F, t1470: F, t1924: F, t1988: F, t1992: F, t344: F, t486: F) -> (F, F) {
    let t1996 = t542 * t1995;
    let t2001 = t1470 + F::cast_from(0.65704296666666666667e-3_f64) * t1102 * t1988 + F::cast_from(0.1478346675e-2_f64) * t344 * t1992 - F::cast_from(0.98556445e-3_f64) * t344 * t1996 - F::cast_from(4.0_f64) * t486 * t1924;
    (t1996, t2001)
}
