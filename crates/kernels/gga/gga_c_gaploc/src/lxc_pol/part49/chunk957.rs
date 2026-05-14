//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 957/1028 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk957<F: Float>(t11016: F, t12256: F, t12207: F, t2718: F, t38947: F, t955: F, t2714: F, t13861: F, t2103: F, t4673: F, t43750: F, t43752: F, t43754: F, t43757: F, t43759: F, t47357: F, t47360: F) -> (F,) {
    let t47362 = t12256 * t11016;
    let t47364 = t2718 * t12207;
    let t47366 = t955 * t38947;
    let t47368 = t2714 * t12207;
    let t47371 = t2103 * t4673 * t13861;
    let t47373 = -t43750 - t43752 - 0.11502877786176224903e2 * t47357 + 0.27606906686822939767e2 * t47360 + t43754 - 0.7150097990370085334e0 * t47362 + t43757 + 0.35750489951850426669e0 * t47364 + 0.35750489951850426669e0 * t47366 + 0.35750489951850426669e0 * t47368 + 0.47667319935800568892e0 * t47371 - t43759;
    (t47373,)
}
