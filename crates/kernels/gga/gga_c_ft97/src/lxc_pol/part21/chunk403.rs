//! GGA_C_FT97 lxc pol — lxc_pol part 21 (v4rho3sigma_6) CSE chunk 403/1339 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part21_v4rho3sigma_6_chunk403<F: Float>(t1985: F, t4668: F, t27: F, t89: F, t1008: F, t132: F, t139: F) -> (F, F, F, F, F) {
    let t4669 = t1985 * t4668;
    let t4671 = t89 * t27 * t4669;
    let t4673 = t1008 * t1008;
    let t4674 = t4673 * t132;
    let t4675 = t4674 * t139;
    (t4669, t4671, t4673, t4674, t4675)
}
