//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1212/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1212<F: Float>(t66166: F, t6757: F, t172: F, t30726: F, t6820: F, t6815: F, t108685: F, t6043: F, t6824: F, t4917: F, t679: F, t689: F, t30852: F, t96694: F, t24378: F, t30625: F, t6034: F) -> (F, F, F, F, F, F, F) {
    let t122787 = t6757 * t66166;
    let t122796 = t30726 * t172 * t6820;
    let t122797 = t6815 * t122796;
    let t122800 = t6043 * t108685 * t6824;
    let t122802 = t4917 * t679;
    let t122803 = t122802 * t689;
    let t122820 = t30852 * t96694;
    let t122824 = t6034 * t24378 * t30625;
    (t122787, t122796, t122797, t122800, t122803, t122820, t122824)
}
