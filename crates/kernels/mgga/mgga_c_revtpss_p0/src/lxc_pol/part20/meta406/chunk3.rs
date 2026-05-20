//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1504/1798 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1504<F: Float>(t11960: F, t351: F, t361: F, t369: F, t1041: F, t11262: F, t3135: F, t1033: F, t1036: F, t1038: F, t1042: F, t1047: F, t1065: F, t1068: F, t11173: F, t11233: F, t11281: F, t11286: F, t11656: F, t11845: F, t11983: F, t2853: F, t3059: F, t3106: F, t3127: F, t3130: F, t3181: F, t42571: F, t4837: F, t906: F) -> F {
    let t42576 = t351 * t361 * t11960 * t369;
    let t42580 = t1041 * t11262 * t3135;
    let t42584 = t1033 * t1036 * t11960 * t1038;
    let t42602 = F::cast_from(0.28582678745379824648e-2_f64) * t4837 * t1042 * t3181 * t3059 * t2853 + F::cast_from(0.18292914397043087775e-1_f64) * t42571 * t3130 - F::cast_from(0.14160070774007427203e0_f64) * t42576 * t1068 - F::cast_from(0.28582678745379824648e-3_f64) * t42580 - F::cast_from(0.21240106161011140804e0_f64) * t42584 * t1047 - F::cast_from(0.57165357490759649296e-3_f64) * t3127 * t1042 * t1065 * t11173 * t906 + F::cast_from(0.91464571985215438872e-2_f64) * t11656 * t11281 + F::cast_from(0.18292914397043087775e-1_f64) * t3106 * t11233 - F::cast_from(0.15244095330869239812e-1_f64) * t3106 * t11983 + F::cast_from(0.15244095330869239812e-1_f64) * t11656 * t11286 - F::cast_from(0.30488190661738479624e-2_f64) * t3106 * t11845;
    t42602
}
