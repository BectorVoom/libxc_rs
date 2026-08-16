//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 1991/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1991<F: Float>(t100572: F, t101226: F, t101832: F, t1484: F, t1530: F, t16596: F, t1877: F, t193: F, t202: F, t24191: F, t24339: F, t24344: F, t2522: F, t26740: F, t26744: F, t28248: F, t29106: F, t29125: F, t4255: F, t4303: F, t4314: F, t46341: F, t5660: F, t67123: F, t7114: F, t776: F, t7845: F, t84766: F, t868: F, t870: F, t92276: F, t97999: F, t98003: F, t98102: F) -> F {
    let t101937 = t193 * t202 * t101832 * t870 + F::cast_from(6.0_f64) * t2522 * t26740 * t1484 + F::cast_from(6.0_f64) * t46341 * t29125 + F::cast_from(2.0_f64) * t1877 * t24344 * t98102 - t1877 * t24339 * t5660 + F::cast_from(12.0_f64) * t4314 * t7845 * t4255 - F::cast_from(6.0_f64) * t1877 * t84766 * t97999 - F::cast_from(6.0_f64) * t2522 * t26744 * t16596 - F::cast_from(2.0_f64) * t1877 * t92276 * t1530 - t1877 * t101226 * t868 + F::cast_from(6.0_f64) * t2522 * t24344 * t98003 + F::cast_from(3.0_f64) * t2522 * t29106 * t776 - F::cast_from(6.0_f64) * t2522 * t24339 * t28248 + F::cast_from(12.0_f64) * t24191 * t100572 - F::cast_from(3.0_f64) * t2522 * t7114 * t67123 - F::cast_from(2.0_f64) * t1877 * t26744 * t4303;
    t101937
}
