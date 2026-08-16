//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1909/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1909(t3403: f64, t4857: f64, t1155: f64, t3395: f64, t4861: f64, t11285: f64, t1694: f64, t3377: f64, t1683: f64, t3333: f64, t11303: f64, t11310: f64, t11415: f64, t15050: f64, t15053: f64, t15056: f64, t15059: f64, t15063: f64, t15066: f64, t15070: f64, t3357: f64, t3401: f64, t4802: f64, t4824: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t15218 = t4857 * t3403;
    let t15219 = t15218 * t1155;
    let t15222 = t4861 * t3395;
    let t15225 = t1694 * t11285;
    let t15226 = t15225 * t3377;
    let t15229 = t1683 * t3333;
    let t15232 = -t15050 + t15053 + t15056 + t15059 - t15063 - t15066 - t15070 - 4.0_f64 * t11303 * t4802 + 0.64327917994770140268e2_f64 * t11415 * t4824 + 0.34631718211362927518e2_f64 * t3401 * t15219 + 0.17315859105681463759e2_f64 * t3401 * t15222 + 0.10254018858216406658e4_f64 * t11310 * t15226 + 6.0_f64 * t3357 * t15229;
    (t15218, t15219, t15222, t15225, t15226, t15229, t15232)
}
