//! MGGA_C_REVTPSS lxc pol — lxc_pol part 22 (v4rho4_2) CSE chunk 3778/3938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part22_v4rho4_2_chunk3778<F: Float>(t3781: F, t6564: F, t20800: F, t3302: F, t13141: F, t1811: F, t460: F, t1204: F, t12723: F, t12751: F, t12756: F, t1285: F, t1287: F, t12966: F, t16696: F, t17192: F, t17454: F, t17856: F, t17864: F, t17902: F, t17955: F, t20703: F, t20850: F, t21459: F, t21465: F, t21468: F, t21513: F, t21518: F, t21527: F, t21592: F, t21607: F, t3670: F, t3727: F, t3746: F, t3759: F, t3760: F, t3784: F, t45634: F, t45683: F, t6622: F) -> F {
    let t72326 = t6564 * t3781;
    let t72329 = t20800 * t3302;
    let t72343 = t460 * t13141 * t1811;
    let t72358 = F::cast_from(0.13170898365871023197e1_f64) * t1204 * t21527 + F::cast_from(0.26341796731742046394e1_f64) * t3670 * t3759 * t20703 + F::cast_from(0.65854491829355115987e0_f64) * t1285 * t3727 * t6622 * t1287 - F::cast_from(0.65854491829355115987e0_f64) * t72326 * t3784 - F::cast_from(0.26341796731742046394e1_f64) * t12751 * t72329 * t17454 + F::cast_from(0.13170898365871023197e1_f64) * t12756 * t72329 * t16696 - F::cast_from(0.52683593463484092788e1_f64) * t45683 * t21513 + F::cast_from(0.26341796731742046394e1_f64) * t45634 * t21518 + F::cast_from(0.52683593463484092788e1_f64) * t12966 * t21592 - F::cast_from(0.79025390195226139182e1_f64) * t72343 * t17856 - F::cast_from(0.26341796731742046394e1_f64) * t17192 * t17902 + F::cast_from(0.26341796731742046394e1_f64) * t17955 * t21465 - F::cast_from(0.13170898365871023197e1_f64) * t17864 * t21468 - F::cast_from(0.13170898365871023197e1_f64) * t12723 * t21459 - F::cast_from(0.13170898365871023197e1_f64) * t20850 * t3760 + F::cast_from(0.26341796731742046394e1_f64) * t3746 * t21607;
    t72358
}
