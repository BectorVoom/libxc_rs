//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 297/1438 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk297<F: Float>(t344: F, t883: F, t607: F, t977: F, t906: F, t910: F) -> (F, F, F, F) {
    let t978 = t344 * t883;
    let t979 = t978 * t607;
    let t980 = t977 * t979;
    let t984 = t906 / F::cast_from(6.0_f64) + t910 / F::cast_from(6.0_f64);
    (t978, t979, t980, t984)
}
