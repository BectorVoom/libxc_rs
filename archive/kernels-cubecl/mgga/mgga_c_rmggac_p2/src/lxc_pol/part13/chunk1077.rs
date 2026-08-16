//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 1077/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk1077<F: Float>(t40560: F, t40562: F, t40578: F, t275: F, t9677: F, t1550: F, t2211: F, t27111: F, t35795: F, t37860: F, t37866: F, t4041: F, t40564: F, t40566: F, t40568: F, t40573: F, t40607: F, t40610: F, t40614: F, t40619: F, t5016: F, t9315: F, t9370: F) -> F {
    let t43466 = F::cast_from(0.1489760996265424379e-3_f64) * t40560;
    let t43467 = F::cast_from(0.1489760996265424379e-3_f64) * t40562;
    let t43472 = F::cast_from(0.15965655602485078085e0_f64) * t40578;
    let t43481 = F::cast_from(2.0_f64) * t275 * t9677;
    let t43488 = t43466 - t43467 - F::cast_from(0.49658699875514145966e-4_f64) * t40564 + F::cast_from(0.49658699875514145966e-4_f64) * t40566 + F::cast_from(0.212822999466489197e-4_f64) * t40568 + F::cast_from(0.212822999466489197e-4_f64) * t40573 - t43472 - F::cast_from(0.23948483403727617128e0_f64) * t5016 * t9315 + F::cast_from(0.23948483403727617128e0_f64) * t1550 * t2211 * t27111 + F::cast_from(0.15965655602485078085e0_f64) * t35795 + t37860 + F::cast_from(0.20455996240684006298e-1_f64) * t40607 + t43481 + F::cast_from(0.11974241701863808564e0_f64) * t4041 * t9370 - F::cast_from(0.5987120850931904282e-1_f64) * t40610 + F::cast_from(0.20455996240684006298e-1_f64) * t40614 - F::cast_from(0.4726e1_f64) * t37866 - F::cast_from(0.638468998399467591e-4_f64) * t40619;
    t43488
}
