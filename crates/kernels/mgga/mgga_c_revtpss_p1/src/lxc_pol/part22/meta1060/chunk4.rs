//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3775/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3775<F: Float>(t1234: F, t12713: F, t12723: F, t1280: F, t1281: F, t1285: F, t1287: F, t17170: F, t17192: F, t17829: F, t17853: F, t17864: F, t17944: F, t1811: F, t20800: F, t21442: F, t21448: F, t21471: F, t21473: F, t21507: F, t21541: F, t21558: F, t3552: F, t3584: F, t45634: F, t45666: F, t45868: F, t5284: F, t5332: F, t5412: F, t5463: F, t5478: F, t59650: F, t59941: F, t6717: F, t6741: F, t70202: F, t70209: F, t70718: F) -> F {
    let t72231 = -F::cast_from(0.15805078039045227836e2_f64) * t17853 * t59650 * t70718 - F::cast_from(0.26341796731742046394e1_f64) * t17864 * t21558 + F::cast_from(0.13170898365871023197e1_f64) * t59941 * t21507 + F::cast_from(0.13170898365871023197e1_f64) * t45634 * t21473 + F::cast_from(0.13170898365871023197e1_f64) * t5463 * t20800 * t12713 - F::cast_from(0.65854491829355115987e0_f64) * t1234 * t21541 * t3584 - F::cast_from(0.26341796731742046394e1_f64) * t17192 * t17829 + F::cast_from(0.65854491829355115987e0_f64) * t3552 * t6741 - F::cast_from(0.13170898365871023197e1_f64) * t45868 * t6717 - F::cast_from(0.13170898365871023197e1_f64) * t5478 * t5332 * t21471 * t17170 - F::cast_from(0.65854491829355115987e0_f64) * t1234 * t1280 * t70202 - F::cast_from(0.79025390195226139182e1_f64) * t45666 * t21442 * t17944 + F::cast_from(0.13170898365871023197e1_f64) * t1285 * t1811 * t17170 * t1287 + F::cast_from(0.26341796731742046394e1_f64) * t1285 * t5412 * t5284 * t1287 - F::cast_from(0.26341796731742046394e1_f64) * t12723 * t21448 - F::cast_from(0.13170898365871023197e1_f64) * t70209 * t1281;
    t72231
}
