//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 691/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk691<F: Float>(t5363: F, t5364: F, t1726: F, t1727: F, t608: F, t1859: F, t766: F, t2: F, t636: F, t1758: F, t188: F, t1907: F) -> (F, F, F, F, F) {
    let t5366 = F::new(0.5143752e0) * t5363 * t5364;
    let t5373 = t1726 * t608 * t1727;
    let t5375 = t1859 * t766;
    let t5376 = t636 * t2;
    let t5377 = t5376 * t1758;
    let t5378 = t5375 * t5377;
    let t5380 = t1907 * t188;
    (t5366, t5373, t5377, t5378, t5380)
}
