//! GGA_C_OPTC lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 1246/1272 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_optc_lxc_pol_part7_v4rho4_0_chunk1246<F: Float>(t1115: F, t1162: F, t3902: F, t1179: F, t26890: F, t27017: F, t27048: F, t27105: F, t27148: F, t27153: F, t27223: F, t27233: F, t27341: F, t27365: F, t27465: F, t27493: F, t27510: F, t27513: F, t27517: F, t27528: F, t27533: F, t27537: F, t27541: F, t3106: F, t3234: F, t3235: F, t3244: F, t3245: F, t4435: F, t4437: F, t4457: F, t4464: F, t8471: F, t9049: F, t9122: F, t914: F, t9175: F) -> (F,) {
    let t27547 = t1162 * t3902 * t1115;
    let t27549 = -0.20408653907080965924e7 * t9122 * t27365 * t27493 - 0.5392791351917231181e5 * t9175 * t3106 * t8471 - 0.45440405106024376544e1 * t3244 * t3245 * t27465 + 0.17581974682482873924e4 * t4464 * t9049 * t27148 - 0.35163949364965747848e4 * t4457 * t9049 * t27153 - 0.6237918122117623248e2 * t27510 + 0.15146801702008125515e1 * t27513 - 0.10097867801338750343e1 * t27517 + 0.11590881986385010473e0 * t1162 * t914 * t27105 + 0.25190352229182098644e-1 * t1179 * t27233 + 0.93568771831764348721e2 * t3234 * t3235 * t27341 - 0.3029360340401625103e1 * t27528 + 0.17581974682482873924e4 * t4457 * t9049 * t27017 + 0.23442632909977165232e4 * t4457 * t27533 * t26890 + 0.80609127133382715661e-1 * t27537 - 0.1209136907000740735e0 * t1179 * t27223 + 0.59710464543246456046e-2 * t27541 + 0.9291736872898228042e2 * t4435 * t27048 * t4437 + 0.42929192542166705456e-1 * t27547;
    (t27549,)
}
