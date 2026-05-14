//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 909/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk909<F: Float>(t10570: F, t10572: F, t10574: F, t10576: F, t16006: F, t16019: F, t16024: F, t16028: F, t16045: F, t16048: F, t16072: F, t11352: F, t11355: F, t11361: F, t15970: F, t15975: F, t15978: F, t16034: F, t16037: F, t16040: F, t1648: F, t1815: F, t2372: F, t4624: F, t4652: F, t4664: F, t4667: F, t574: F, t6750: F, t6771: F, t6774: F) -> (F,) {
    let t16073 = 0.1982e-1 * t16045 + 0.14865e-1 * t16048 + 0.1651e-1 * t16006 - 0.24765e-1 * t16019 - 0.27516666666666666666e-2 * t10574 + 0.8255e-2 * t16028 - 0.3302e-1 * t16024 - 0.36688888888888888888e-2 * t10570 + 0.13758333333333333333e-2 * t10576 + 0.9172222222222222222e-3 * t10572 + t16072;
    let t16076 = 3.0 / 16.0 * t11352 * t15970 - t11355 * t6750 / 4.0 - t4664 * t15975 / 4.0 - t4664 * t15978 / 8.0 + t11361 * t2372 / 4.0 + t4667 * t6771 / 2.0 + t1815 * t16034 / 4.0 - t16037 * t4624 / 8.0 + t16040 * t1648 / 2.0 + t6774 * t4652 / 4.0 + t574 * t16073 / 2.0;
    (t16076,)
}
