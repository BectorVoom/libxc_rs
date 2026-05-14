//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 879/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk879<F: Float>(t27882: F, t713: F, t193: F, t89: F, t27844: F, t27848: F, t27853: F, t27858: F, t27861: F, t27864: F, t27867: F, t27870: F, t27873: F, t27876: F, t27881: F, t27761: F, t27794: F, t27840: F) -> (F, F, F) {
    let t27883 = t27882 * t713;
    let t27884 = t193 * t27883;
    let t27885 = t89 * t27884;
    let t27887 = t27844 + t27848 / 4.0 + t27853 / 4.0 + t27858 / 4.0 - 2.0 / 3.0 * t27861 - 2.0 / 3.0 * t27864 - 2.0 / 3.0 * t27867 + 2.0 / 9.0 * t27870 - t27873 / 12.0 - t27876 / 3.0 + t27881 + 2.0 * t27885;
    let t27889 = t27761 + t27794 + t27840 + t27887;
    (t27883, t27885, t27889)
}
