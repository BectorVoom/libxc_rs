//! GGA_C_GAPLOC lxc pol — lxc_pol part 46 (v4rhosigma3_11) CSE chunk 990/1029 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part46_v4rhosigma3_11_chunk990<F: Float>(t10915: F, t22242: F, t43598: F, t2684: F, t43486: F, t7585: F, t43750: F, t43752: F, t43754: F, t43757: F, t43759: F, t43761: F, t43762: F, t43766: F, t43768: F, t43771: F, t43774: F, t43775: F, t43776: F, t43777: F, t43778: F, t43781: F, t43783: F, t43787: F) -> F {
    let t43790 = F::cast_from(0.21450293971110256001e1_f64) * t22242 * t10915 * t43598;
    let t43793 = F::cast_from(0.87421871174939309262e2_f64) * t2684 * t7585 * t43486;
    let t43794 = -t43750 - t43752 + t43754 + t43757 - t43759 + t43761 + F::cast_from(0.23833659967900284447e0_f64) * t43762 + t43766 + F::cast_from(0.85801175884441024008e1_f64) * t43768 - F::cast_from(0.42900587942220512004e1_f64) * t43771 + t43774 + t43775 - t43776 + t43777 - t43778 + t43781 - t43783 - t43787 + t43790 + t43793;
    t43794
}
