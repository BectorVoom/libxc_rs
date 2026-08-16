//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2581/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2581(t11570: f64, t12652: f64, t1174: f64, t1709: f64, t44633: f64, t11530: f64, t4889: f64, t15273: f64, t15281: f64, t11533: f64, t11496: f64, t11502: f64, t11510: f64, t11518: f64, t11522: f64, t11569: f64, t1177: f64, t1178: f64, t1714: f64, t3447: f64, t3475: f64, t44512: f64, t44527: f64, t44564: f64, t44573: f64, t44581: f64, t45872: f64, t460: f64, t4928: f64, t4934: f64) -> f64 {
    let t52271 = t11570 * t12652;
    let t52281 = t1174 * t44633 * t1709;
    let t52288 = t4889 * t11530;
    let t52296 = t1174 * t15281 * t15273;
    let t52300 = t4889 * t11533;
    let t52303 = 0.27777777777777777777e-3_f64 * t44512 - 0.83333333333333333332e-3_f64 * t1174 * t4934 * t1714 * t11496 * t460 + 0.44444444444444444445e-2_f64 * t4889 * t11522 - 0.27777777777777777777e-3_f64 * t1174 * t1177 * t1178 * t45872 - 0.22222222222222222221e-2_f64 * t3447 * t11569 * t52271 - 0.37037037037037037036e-3_f64 * t44527 - 0.59259259259259259259e-2_f64 * t4889 * t11518 + 0.66666666666666666666e-2_f64 * t4889 * t11510 - 0.10288065843621399177e-3_f64 * t52281 - 0.83333333333333333332e-3_f64 * t1174 * t4934 * t1714 * t11502 * t460 - 0.49382716049382716048e-3_f64 * t52288 - 0.24999999999999999999e-2_f64 * t1174 * t4934 * t4928 * t3475 * t460 - 0.83333333333333333331e-3_f64 * t52296 - 0.28806584362139917695e-3_f64 * t44564 - 0.24691358024691358024e-3_f64 * t44573 + 0.74074074074074074072e-3_f64 * t52300 + 0.27777777777777777777e-3_f64 * t44581;
    t52303
}
