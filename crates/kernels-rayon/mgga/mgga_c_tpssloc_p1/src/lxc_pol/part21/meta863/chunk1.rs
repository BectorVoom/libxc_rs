//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 3142/3221 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk3142(t15299: f64, t4889: f64, t15285: f64, t12652: f64, t14725: f64, t17686: f64, t44505: f64, t15363: f64, t1174: f64, t15281: f64, t18549: f64, t1090: f64, t1184: f64, t15304: f64, t15376: f64, t15383: f64, t15395: f64, t18523: f64, t27654: f64, t3440: f64, t3441: f64, t3447: f64, t44504: f64, t460: f64, t4919: f64, t4934: f64, t52191: f64, t55723: f64) -> (f64, f64) {
    let t65002 = t4889 * t15299;
    let t65008 = t4889 * t15285;
    let t65014 = t14725 * t12652;
    let t65018 = t44505 * t17686;
    let t65023 = t4889 * t15363;
    let t65035 = t1174 * t15281 * t18549;
    let t65037 = -0.32921810699588477365e-3_f64 * t65002 + 0.74074074074074074072e-3_f64 * t1174 * t3440 * t3441 * t55723 + 0.98765432098765432094e-3_f64 * t65008 + 0.11111111111111111111e-2_f64 * t3447 * t4919 * t27654 * t1090 - 0.34567901234567901234e-2_f64 * t3447 * t15395 * t65014 + 0.17283950617283950617e-2_f64 * t3447 * t44504 * t65018 + 0.37037037037037037036e-3_f64 * t52191 - 0.987654320987654321e-3_f64 * t65023 + 0.39506172839506172838e-2_f64 * t15376 * t15383 + 0.44444444444444444444e-2_f64 * t4889 * t15304 - 0.16666666666666666666e-2_f64 * t1174 * t4934 * t18523 * t1184 * t460 - 0.11111111111111111111e-2_f64 * t65035;
    (t65014, t65037)
}
