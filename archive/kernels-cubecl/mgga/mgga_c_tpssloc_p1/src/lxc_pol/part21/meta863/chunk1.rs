//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3142/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3142<F: Float>(t15299: F, t4889: F, t15285: F, t12652: F, t14725: F, t17686: F, t44505: F, t15363: F, t1174: F, t15281: F, t18549: F, t1090: F, t1184: F, t15304: F, t15376: F, t15383: F, t15395: F, t18523: F, t27654: F, t3440: F, t3441: F, t3447: F, t44504: F, t460: F, t4919: F, t4934: F, t52191: F, t55723: F) -> (F, F) {
    let t65002 = t4889 * t15299;
    let t65008 = t4889 * t15285;
    let t65014 = t14725 * t12652;
    let t65018 = t44505 * t17686;
    let t65023 = t4889 * t15363;
    let t65035 = t1174 * t15281 * t18549;
    let t65037 = -F::cast_from(0.32921810699588477365e-3_f64) * t65002 + F::cast_from(0.74074074074074074072e-3_f64) * t1174 * t3440 * t3441 * t55723 + F::cast_from(0.98765432098765432094e-3_f64) * t65008 + F::cast_from(0.11111111111111111111e-2_f64) * t3447 * t4919 * t27654 * t1090 - F::cast_from(0.34567901234567901234e-2_f64) * t3447 * t15395 * t65014 + F::cast_from(0.17283950617283950617e-2_f64) * t3447 * t44504 * t65018 + F::cast_from(0.37037037037037037036e-3_f64) * t52191 - F::cast_from(0.987654320987654321e-3_f64) * t65023 + F::cast_from(0.39506172839506172838e-2_f64) * t15376 * t15383 + F::cast_from(0.44444444444444444444e-2_f64) * t4889 * t15304 - F::cast_from(0.16666666666666666666e-2_f64) * t1174 * t4934 * t18523 * t1184 * t460 - F::cast_from(0.11111111111111111111e-2_f64) * t65035;
    (t65014, t65037)
}
