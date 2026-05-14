//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 863/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk863<F: Float>(t1882: F, t22446: F, t1775: F, t22323: F, t22287: F, t22294: F, t1196: F, t283: F, t21249: F, t21253: F, t280: F, t22071: F, t22059: F, t816: F, t2724: F, t21130: F, t2344: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t82638 = t1882 * t22446;
    let t82769 = t1775 * t22323;
    let t82771 = t1775 * t22287;
    let t82773 = t1775 * t22294;
    let t82816 = t1196 * t283;
    let t82845 = t280 * t21249 * t21253;
    let t82848 = t22071 * t21253;
    let t82851 = t816 * t22059;
    let t82855 = t2724 * t22059;
    let t82988 = t2344 * t21130;
    (t82638, t82769, t82771, t82773, t82816, t82845, t82848, t82851, t82855, t82988)
}
