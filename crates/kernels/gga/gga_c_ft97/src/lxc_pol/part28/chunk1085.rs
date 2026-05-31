//! GGA_C_FT97 lxc pol — lxc_pol part 28 (v4rho2sigma2_6) CSE chunk 1085/1189 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part28_v4rho2sigma2_6_chunk1085<F: Float>(t1882: F, t34634: F, t34624: F, t34746: F, t34742: F, t34762: F, t103654: F, t110: F, t11810: F, t1307: F, t137891: F, t137900: F, t137906: F, t145658: F, t145705: F, t1871: F, t1901: F, t23339: F, t26061: F, t26113: F, t26134: F, t26154: F, t26198: F, t26445: F, t3103: F, t3113: F, t32082: F, t32333: F, t32545: F, t3271: F, t39120: F, t446: F, t452: F, t47659: F, t488: F, t5644: F, t7281: F, t83: F, t91739: F, t986: F) -> F {
    let t146498 = t1882 * t34634;
    let t146505 = t1882 * t34624;
    let t146520 = t1882 * t34746;
    let t146522 = t1882 * t34742;
    let t146527 = t1882 * t34762;
    let t146547 = F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t446 * t1871 * t986 * t32082 + t137891 / F::cast_from(9.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t137900 + t446 * t452 * t32545 * t3271 / F::cast_from(3.0_f64) + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t146498 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t452 * t488 * t1307 * t26113 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t146505 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t137906 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1901 * t39120 * t32333 * t3113 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t1901 * t11810 * t23339 * t26154 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t1871 * t110 * t145658 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t146520 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t146522 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t47659 * t91739 * t26198 + t146527 / F::cast_from(9.0_f64) + t446 * t452 * t488 * t7281 * t3103 / F::cast_from(3.0_f64) - t446 * t83 * t145705 / F::cast_from(3.0_f64) + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t47659 * t91739 * t26445 + F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t47659 * t103654 * t26134 + F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t446 * t452 * t26061 * t5644;
    t146547
}
