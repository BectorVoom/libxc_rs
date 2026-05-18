//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 610/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk610<F: Float>(t27855: F, t743: F, t193: F, t6109: F, t27767: F, t9770: F, t446: F, t27753: F, t2354: F, t27757: F, t27763: F, t9744: F) -> (F, F, F, F, F, F, F, F) {
    let t27856 = t743 * t27855;
    let t27858 = t6109 * t193 * t27856;
    let t27860 = t9770 * t27767;
    let t27861 = t446 * t27860;
    let t27863 = t9770 * t27753;
    let t27864 = t446 * t27863;
    let t27866 = t2354 * t27757;
    let t27867 = t446 * t27866;
    let t27869 = t9744 * t27763;
    (t27858, t27860, t27861, t27863, t27864, t27866, t27867, t27869)
}
