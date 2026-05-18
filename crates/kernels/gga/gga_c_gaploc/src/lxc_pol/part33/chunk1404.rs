//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 1404/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk1404<F: Float>(t12038: F, t12065: F, t12116: F, t12117: F, t12128: F, t1625: F, t1641: F, t193: F, t35123: F, t35126: F, t35128: F, t35130: F, t35133: F, t35136: F, t35138: F, t35140: F, t35142: F, t35144: F, t35146: F, t4379: F, t524: F, t541: F) -> F {
    let t38836 = -t35123 + t35126 + F::new(0.79445533226334281486e-1) * t4379 * t12038 + t35128 - t35130 - t35133 + t35136 - t35138 + t35140 + t35142 + t35144 - t35146 - F::new(0.61348681526273199482e1) * t1641 * t12128 + F::new(0.71500979903700853338e0) * t524 * t12116 * t193 + F::new(0.47667319935800568892e0) * t12117 * t541 + F::new(0.35750489951850426669e0) * t1625 * t12065;
    t38836
}
