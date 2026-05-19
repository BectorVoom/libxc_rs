//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 788/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk788<F: Float>(t2148: F, t7629: F, t7628: F, t1398: F, t5: F, t966: F, t2804: F, t378: F, t1707: F, t898: F, t1726: F, t1727: F, t956: F) -> (F, F, F, F, F) {
    let t7630 = t2148 * t7629;
    let t7632 = F::cast_from(0.23287303101564395622e-1_f64) * t7628 * t7630;
    let t7637 = t5 * t1398 * t966;
    let t7641 = F::new(10.0) / F::new(3.0) * t5 * t378 * t2804;
    let t7647 = t898 * t1707;
    let t7650 = t1726 * t956 * t1727;
    (t7632, t7637, t7641, t7647, t7650)
}
