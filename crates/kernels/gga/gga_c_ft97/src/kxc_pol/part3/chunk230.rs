//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 230/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk230<F: Float>(t762: F, t766: F, t242: F, t193: F, t446: F, t723: F, t726: F, t731: F, t756: F, t89: F, t258: F, t754: F) -> (F, F, F, F) {
    let t767 = t762 * t766;
    let t768 = t242 * t767;
    let t771 = -t723 - t446 * t726 / 9.0 - t446 * t731 / 3.0 + t89 * t193 * t756 / 3.0 - t446 * t768 / 3.0;
    let t773 = t754 * t258;
    (t767, t768, t771, t773)
}
