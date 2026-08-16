//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2417/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2417<F: Float>(t10811: F, t1568: F, t14255: F, t892: F, t914: F, t2791: F, t4351: F, t2794: F, t10660: F, t1543: F, t10663: F, t10603: F, t10747: F, t10813: F, t10825: F, t10828: F, t14344: F, t14366: F, t14370: F, t14453: F, t14456: F, t14459: F, t14460: F, t1581: F, t2862: F, t2880: F, t2886: F, t2905: F, t2906: F, t2924: F, t41816: F, t41821: F, t42128: F, t4434: F, t4472: F, t4476: F, t931: F, t950: F) -> (F, F, F, F) {
    let t49478 = t10811 * t1568;
    let t49483 = t14255 * t892;
    let t49485 = F::cast_from(3.0_f64) * t49483 * t914;
    let t49486 = t4351 * t2791;
    let t49488 = F::cast_from(6.0_f64) * t49486 * t2794;
    let t49489 = t1543 * t10660;
    let t49491 = F::cast_from(0.96491876992155210402e2_f64) * t49489 * t10663;
    let t49492 = -F::cast_from(0.35089341735807877242e1_f64) * t10747 * t14453 - F::cast_from(0.31168546390226634765e3_f64) * t42128 * t14456 + F::cast_from(0.51947577317044391277e2_f64) * t41816 * t4476 + F::cast_from(0.10389515463408878255e3_f64) * t10825 * t14460 + F::cast_from(0.51947577317044391277e2_f64) * t10825 * t14366 + F::cast_from(0.30762056574649219973e4_f64) * t41821 * t14370 - F::cast_from(0.35089341735807877242e1_f64) * t2905 * t14344 * t950 - F::cast_from(0.35089341735807877242e1_f64) * t2905 * t4472 * t2924 - F::cast_from(0.31168546390226634765e3_f64) * t10828 * t14459 * t2906 - F::cast_from(0.11696447245269292414e1_f64) * t2905 * t1581 * t10603 + F::cast_from(18.0_f64) * t2886 * t4434 * t2862 + F::cast_from(0.6207121550312808036e4_f64) * t49478 * t10813 * t2880 * t931 - t49485 + t49488 + t49491;
    (t49485, t49488, t49491, t49492)
}
