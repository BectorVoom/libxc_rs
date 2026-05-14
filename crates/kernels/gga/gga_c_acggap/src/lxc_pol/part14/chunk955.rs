//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 955/1080 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk955<F: Float>(t33997: F, t34011: F, t34014: F, t34024: F, t34028: F, t34030: F, t34032: F, t34036: F, t34038: F, t36898: F, t36911: F, t36914: F, t38909: F, t38912: F, t38914: F, t38916: F, t38920: F, t38925: F) -> (F,) {
    let t38927 = t33997 + 0.94344276868812456204e-3 * t38909 + t36898 - 0.83861579438944405513e-2 * t34011 + t34014 - 0.85748036236139473944e-3 * t38912 + t34024 + t34028 + t34030 - t34032 - t36911 - t34036 - t34038 - t36914 - 0.53592522647587171215e-3 * t38914 + 0.20007875121765877254e-2 * t38916 - 0.53592522647587171215e-3 * t38920 - 0.53592522647587171215e-3 * t38925;
    (t38927,)
}
