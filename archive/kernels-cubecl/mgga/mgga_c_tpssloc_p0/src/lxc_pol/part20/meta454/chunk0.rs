//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1909/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1909<F: Float>(t3403: F, t4857: F, t1155: F, t3395: F, t4861: F, t11285: F, t1694: F, t3377: F, t1683: F, t3333: F, t11303: F, t11310: F, t11415: F, t15050: F, t15053: F, t15056: F, t15059: F, t15063: F, t15066: F, t15070: F, t3357: F, t3401: F, t4802: F, t4824: F) -> (F, F, F, F, F, F, F) {
    let t15218 = t4857 * t3403;
    let t15219 = t15218 * t1155;
    let t15222 = t4861 * t3395;
    let t15225 = t1694 * t11285;
    let t15226 = t15225 * t3377;
    let t15229 = t1683 * t3333;
    let t15232 = -t15050 + t15053 + t15056 + t15059 - t15063 - t15066 - t15070 - F::cast_from(4.0_f64) * t11303 * t4802 + F::cast_from(0.64327917994770140268e2_f64) * t11415 * t4824 + F::cast_from(0.34631718211362927518e2_f64) * t3401 * t15219 + F::cast_from(0.17315859105681463759e2_f64) * t3401 * t15222 + F::cast_from(0.10254018858216406658e4_f64) * t11310 * t15226 + F::cast_from(6.0_f64) * t3357 * t15229;
    (t15218, t15219, t15222, t15225, t15226, t15229, t15232)
}
