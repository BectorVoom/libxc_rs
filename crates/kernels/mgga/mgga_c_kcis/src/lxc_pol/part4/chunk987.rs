//! MGGA_C_KCIS lxc pol — lxc_pol part 4 (v3rho3_1) CSE chunk 987/1239 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part4_v3rho3_1_chunk987<F: Float>(t13717: F, t13720: F, t13726: F, t13729: F, t13735: F, t13738: F, t13742: F, t13782: F, t9681: F, t9683: F, t9691: F, t10199: F, t10202: F, t10208: F, t1036: F, t13691: F, t13696: F, t13699: F, t13744: F, t13747: F, t13750: F, t1670: F, t245: F, t2944: F, t2952: F, t3078: F, t3081: F, t4625: F, t4647: F, t4654: F, t934: F) -> (F,) {
    let t13783 = -0.27516666666666666667e-2 * t13729 - 0.45861111111111111112e-2 * t13720 - 0.11006666666666666667e-1 * t13726 + 0.8255e-2 * t13738 + 0.3302e-1 * t13735 + 0.30268333333333333334e-1 * t13717 - 0.8255e-2 * t13742 + 0.13758333333333333333e-2 * t9681 + 0.9172222222222222222e-3 * t9683 - 0.36688888888888888888e-2 * t9691 + t13782;
    let t13786 = 3.0 / 16.0 * t10199 * t13691 - t10202 * t4647 / 4.0 - t3078 * t13696 / 4.0 - t3078 * t13699 / 8.0 + t10208 * t1670 / 4.0 + t3081 * t4625 / 2.0 + t1036 * t13744 / 4.0 - t13747 * t2944 / 8.0 + t13750 * t934 / 2.0 + t4654 * t2952 / 4.0 + t245 * t13783 / 2.0;
    (t13786,)
}
