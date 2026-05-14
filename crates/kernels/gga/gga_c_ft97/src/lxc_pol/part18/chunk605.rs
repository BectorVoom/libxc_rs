//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 605/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk605<F: Float>(t1546: F, t1979: F, t89: F, t1965: F, t7780: F, t1987: F, t375: F, t128: F, t39: F, t2035: F) -> (F, F, F, F, F) {
    let t8799 = t89 * t1546 * t1979;
    let t8802 = t89 * t7780 * t1965;
    let t8805 = t89 * t375 * t1987;
    let t8811 = t128 * t39;
    let t8812 = t8811 * t2035;
    (t8799, t8802, t8805, t8811, t8812)
}
