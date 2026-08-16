//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 23 (v4rho4_4) CSE chunk 1444/1527 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part23_v4rho4_4_chunk1444(t6144: f64, t6138: f64, t1174: f64, t1409: f64, t1710: f64, t18321: f64, t22035: f64, t22041: f64, t22056: f64, t22060: f64, t3447: f64, t3450: f64, t457: f64, t460: f64, t4889: f64, t4919: f64, t6131: f64, t65112: f64, t65126: f64, t73113: f64, t974: f64) -> f64 {
    let t78562 = t6144 * t6144;
    let t78568 = t6138 * t6138;
    let t78578 = 0.17777777777777777777e-1_f64 * t4889 * t22060 + 0.50699588477366255142e-1_f64 * t73113 * t1710 - 0.16296296296296296296e-1_f64 * t18321 * t6131 - 0.23703703703703703704e-1_f64 * t4889 * t22056 + 0.33333333333333333332e-2_f64 * t3447 * t4919 * t3450 * t1409 * t6138 + 0.88888888888888888888e-2_f64 * t4889 * t22035 - 0.83333333333333333332e-3_f64 * t1174 * t974 * t457 * t78562 * t460 - 0.24999999999999999999e-2_f64 * t1174 * t974 * t457 * t78568 * t460 + 0.88888888888888888888e-2_f64 * t4889 * t22041 + 0.74074074074074074072e-3_f64 * t65112 - 0.49382716049382716048e-3_f64 * t65126;
    t78578
}
