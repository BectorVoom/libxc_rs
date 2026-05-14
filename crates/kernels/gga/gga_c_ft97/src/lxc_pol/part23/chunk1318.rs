//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 1318/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk1318<F: Float>(t31699: F, t8392: F, t31717: F, t10703: F, t114578: F, t11593: F, t1240: F, t15369: F, t15460: F, t1901: F, t19373: F, t19378: F, t19460: F, t19534: F, t19538: F, t24886: F, t24898: F, t28520: F, t28524: F, t2881: F, t29055: F, t29082: F, t29093: F, t29127: F, t29130: F, t29207: F, t29215: F, t29222: F, t31867: F, t3746: F, t4167: F, t44518: F, t56098: F, t684: F, t69875: F, t69879: F) -> (F,) {
    let t125936 = t8392 * t31699;
    let t125942 = t8392 * t31717;
    let t125950 = -4.0 / 3.0 * t1901 * t15369 * t114578 * t4167 - 2.0 / 9.0 * t1901 * t10703 * t31867 * t684 - 2.0 / 9.0 * t1901 * t56098 * t29215 - 2.0 / 27.0 * t1901 * t44518 * t29207 * t19460 - 2.0 / 3.0 * t1901 * t15369 * t24898 * t19373 - 2.0 / 3.0 * t1901 * t15460 * t29055 * t19378 - 4.0 / 9.0 * t11593 * t2881 * t29082 * t3746 - 2.0 / 9.0 * t1901 * t56098 * t29222 - 4.0 / 9.0 * t1901 * t69875 * t28520 + 4.0 / 27.0 * t1901 * t69879 * t28524 + 2.0 / 27.0 * t125936 - 4.0 * t1901 * t29127 * t1240 * t29130 - 2.0 / 27.0 * t125942 + 2.0 / 9.0 * t1901 * t24886 * t19534 - 2.0 / 27.0 * t1901 * t29093 * t19538;
    (t125950,)
}
