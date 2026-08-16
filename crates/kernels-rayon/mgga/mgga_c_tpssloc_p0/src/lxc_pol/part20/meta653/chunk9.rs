//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2417/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2417(t10811: f64, t1568: f64, t14255: f64, t892: f64, t914: f64, t2791: f64, t4351: f64, t2794: f64, t10660: f64, t1543: f64, t10663: f64, t10603: f64, t10747: f64, t10813: f64, t10825: f64, t10828: f64, t14344: f64, t14366: f64, t14370: f64, t14453: f64, t14456: f64, t14459: f64, t14460: f64, t1581: f64, t2862: f64, t2880: f64, t2886: f64, t2905: f64, t2906: f64, t2924: f64, t41816: f64, t41821: f64, t42128: f64, t4434: f64, t4472: f64, t4476: f64, t931: f64, t950: f64) -> (f64, f64, f64, f64) {
    let t49478 = t10811 * t1568;
    let t49483 = t14255 * t892;
    let t49485 = 3.0_f64 * t49483 * t914;
    let t49486 = t4351 * t2791;
    let t49488 = 6.0_f64 * t49486 * t2794;
    let t49489 = t1543 * t10660;
    let t49491 = 0.96491876992155210402e2_f64 * t49489 * t10663;
    let t49492 = -0.35089341735807877242e1_f64 * t10747 * t14453 - 0.31168546390226634765e3_f64 * t42128 * t14456 + 0.51947577317044391277e2_f64 * t41816 * t4476 + 0.10389515463408878255e3_f64 * t10825 * t14460 + 0.51947577317044391277e2_f64 * t10825 * t14366 + 0.30762056574649219973e4_f64 * t41821 * t14370 - 0.35089341735807877242e1_f64 * t2905 * t14344 * t950 - 0.35089341735807877242e1_f64 * t2905 * t4472 * t2924 - 0.31168546390226634765e3_f64 * t10828 * t14459 * t2906 - 0.11696447245269292414e1_f64 * t2905 * t1581 * t10603 + 18.0_f64 * t2886 * t4434 * t2862 + 0.6207121550312808036e4_f64 * t49478 * t10813 * t2880 * t931 - t49485 + t49488 + t49491;
    (t49485, t49488, t49491, t49492)
}
