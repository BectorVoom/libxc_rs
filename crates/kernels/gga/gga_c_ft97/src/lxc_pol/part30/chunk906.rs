//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 906/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk906<F: Float>(t140920: F, t3729: F, t140919: F, t2393: F, t3733: F, t1609: F, t218: F, t2378: F, t140943: F, t35426: F, t35427: F, t1109: F, t17839: F, t3762: F, t6789: F, t695: F) -> (F, F, F, F, F, F) {
    let t150727 = t140920 * t3729;
    let t150731 = t140919 * t2393 * t3733;
    let t150736 = t1609 * t218 * t2378 * t3733;
    let t150740 = t35426 * t140943 * t35427;
    let t150751 = t17839 * t1109;
    let t150752 = t150751 * t3762;
    let t150755 = t695 * t6789;
    (t150727, t150731, t150736, t150740, t150752, t150755)
}
