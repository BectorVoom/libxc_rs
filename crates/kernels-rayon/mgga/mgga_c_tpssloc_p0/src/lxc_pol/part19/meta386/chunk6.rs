//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1453/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1453(t11525: f64, t1174: f64, t3431: f64, t1176: f64, t2402: f64, t1179: f64, t11529: f64, t3460: f64, t3456: f64, t11516: f64, t11547: f64, t11569: f64, t1177: f64, t1178: f64, t15395: f64, t3440: f64, t3447: f64, t3455: f64, t39097: f64, t39103: f64, t39110: f64, t43711: f64, t43732: f64, t44602: f64, t44608: f64, t44621: f64, t44622: f64, t44628: f64, t4900: f64) -> f64 {
    let t44631 = t1174 * t3431 * t11525;
    let t44633 = t2402 * t1176;
    let t44635 = t1174 * t44633 * t1179;
    let t44638 = t1174 * t11529 * t3460;
    let t44641 = t1174 * t11529 * t3456;
    let t44655 = 0.22222222222222222222e-2_f64 * t44602 + 0.13333333333333333333e-1_f64 * t3447 * t4900 * t43711 - 0.88888888888888888886e-2_f64 * t3447 * t11569 * t44608 - 0.51851851851851851851e-2_f64 * t3447 * t15395 * t43732 - 0.16666666666666666666e-2_f64 * t1174 * t1177 * t3455 * t39103 + 0.28806584362139917695e-2_f64 * t1174 * t44621 * t44622 * t39097 - 0.33333333333333333332e-2_f64 * t44628 - 0.37037037037037037036e-3_f64 * t44631 - 0.41152263374485596707e-3_f64 * t44635 + 0.37037037037037037036e-3_f64 * t44638 + 0.74074074074074074072e-3_f64 * t44641 + 0.13333333333333333332e-1_f64 * t1174 * t3440 * t11547 * t39097 - 0.66666666666666666664e-2_f64 * t1174 * t1177 * t11516 * t39097 - 0.27777777777777777777e-3_f64 * t1174 * t1177 * t1178 * t39110;
    t44655
}
