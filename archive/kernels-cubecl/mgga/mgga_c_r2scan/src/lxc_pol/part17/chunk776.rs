//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 776/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk776<F: Float>(t2719: F, t788: F, t2201: F, t785: F, t2202: F, t2837: F, t1620: F, t2682: F, t129: F, t1598: F, t524: F, t2593: F) -> (F, F, F, F, F) {
    let t7476 = t788 * t2719;
    let t7479 = F::cast_from(0.11643651550782197811e-1_f64) * t2201 * t785 * t7476;
    let t7482 = F::cast_from(0.11643651550782197811e-1_f64) * t2201 * t2837 * t2202;
    let t7490 = t1620 * t2682;
    let t7494 = t524 * t1598 * t129;
    let t7496 = F::cast_from(0.25610080155860322884e0_f64) * t7494 * t2593;
    (t7479, t7482, t7490, t7494, t7496)
}
