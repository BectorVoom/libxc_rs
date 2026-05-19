//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1386/1414 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1386<F: Float>(t27592: F, t9116: F, t9118: F, t9102: F, t9104: F, t1162: F, t1179: F, t12567: F, t12601: F, t12602: F, t12606: F, t12617: F, t12621: F, t27088: F, t27237: F, t27415: F, t27424: F, t27425: F, t27449: F, t27552: F, t27553: F, t27557: F, t27559: F, t27567: F, t27570: F, t27575: F, t27579: F, t27587: F, t27590: F, t27594: F, t3103: F, t3107: F, t3119: F, t3244: F, t3245: F, t4435: F, t4457: F, t4464: F, t8975: F, t9097: F, t914: F) -> F {
    let t27597 = t9116 * t27592 * t9118;
    let t27600 = t9102 * t27592 * t9104;
    let t27614 = -F::cast_from(0.23967961564076583027e5_f64) * t27552 * t27553 * t8975 + F::cast_from(0.35163949364965747848e4_f64) * t12606 * t27557 * t3107 * t27559 - F::cast_from(0.93568771831764348721e2_f64) * t12601 * t12602 * t27449 + F::cast_from(0.26631068404529536697e4_f64) * t27567 + F::cast_from(0.23181763972770020946e0_f64) * t27570 + F::cast_from(0.15486228121497046737e3_f64) * t4435 * t12621 * t27425 + F::cast_from(0.35163949364965747848e4_f64) * t4457 * t9097 * t27575 - F::cast_from(0.17581974682482873924e4_f64) * t4464 * t9097 * t27579 + F::cast_from(0.28977204965962526182e-1_f64) * t1162 * t914 * t27088 + F::cast_from(0.38636273287950034909e-1_f64) * t27587 + F::cast_from(0.31957282085435444036e5_f64) * t27590 - F::cast_from(0.45352564237957702055e6_f64) * t27594 + F::cast_from(0.45352564237957702055e6_f64) * t27597 + F::cast_from(0.75587607063262836759e5_f64) * t27600 - F::cast_from(0.90880810212048753088e1_f64) * t12567 * t12617 * t27449 + F::cast_from(0.61944912485988186947e2_f64) * t3103 * t12602 * t27424 * t3119 + F::cast_from(0.15146801702008125515e1_f64) * t3244 * t3245 * t27415 - F::cast_from(0.30228422675018518374e-1_f64) * t1179 * t27237;
    t27614
}
