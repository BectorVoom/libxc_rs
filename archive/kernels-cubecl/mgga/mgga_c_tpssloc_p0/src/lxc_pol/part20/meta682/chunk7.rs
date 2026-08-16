//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2581/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2581<F: Float>(t11570: F, t12652: F, t1174: F, t1709: F, t44633: F, t11530: F, t4889: F, t15273: F, t15281: F, t11533: F, t11496: F, t11502: F, t11510: F, t11518: F, t11522: F, t11569: F, t1177: F, t1178: F, t1714: F, t3447: F, t3475: F, t44512: F, t44527: F, t44564: F, t44573: F, t44581: F, t45872: F, t460: F, t4928: F, t4934: F) -> F {
    let t52271 = t11570 * t12652;
    let t52281 = t1174 * t44633 * t1709;
    let t52288 = t4889 * t11530;
    let t52296 = t1174 * t15281 * t15273;
    let t52300 = t4889 * t11533;
    let t52303 = F::cast_from(0.27777777777777777777e-3_f64) * t44512 - F::cast_from(0.83333333333333333332e-3_f64) * t1174 * t4934 * t1714 * t11496 * t460 + F::cast_from(0.44444444444444444445e-2_f64) * t4889 * t11522 - F::cast_from(0.27777777777777777777e-3_f64) * t1174 * t1177 * t1178 * t45872 - F::cast_from(0.22222222222222222221e-2_f64) * t3447 * t11569 * t52271 - F::cast_from(0.37037037037037037036e-3_f64) * t44527 - F::cast_from(0.59259259259259259259e-2_f64) * t4889 * t11518 + F::cast_from(0.66666666666666666666e-2_f64) * t4889 * t11510 - F::cast_from(0.10288065843621399177e-3_f64) * t52281 - F::cast_from(0.83333333333333333332e-3_f64) * t1174 * t4934 * t1714 * t11502 * t460 - F::cast_from(0.49382716049382716048e-3_f64) * t52288 - F::cast_from(0.24999999999999999999e-2_f64) * t1174 * t4934 * t4928 * t3475 * t460 - F::cast_from(0.83333333333333333331e-3_f64) * t52296 - F::cast_from(0.28806584362139917695e-3_f64) * t44564 - F::cast_from(0.24691358024691358024e-3_f64) * t44573 + F::cast_from(0.74074074074074074072e-3_f64) * t52300 + F::cast_from(0.27777777777777777777e-3_f64) * t44581;
    t52303
}
