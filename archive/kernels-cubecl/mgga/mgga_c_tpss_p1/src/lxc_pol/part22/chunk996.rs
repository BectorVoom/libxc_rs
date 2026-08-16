//! MGGA_C_TPSS lxc pol — lxc_pol part 22 (v4rho3sigma_4) CSE chunk 996/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpss_lxc_pol_part22_v4rho3sigma_4_chunk996<F: Float>(t8227: F, t2334: F, t3572: F, t1289: F, t2332: F, t681: F, t1351: F, t37: F, t2338: F, t189: F, t3431: F, t581: F) -> (F, F, F, F, F) {
    let t10704 = F::cast_from(0.21687162600603479684e-1_f64) * t8227;
    let t10706 = F::cast_from(8.0_f64) * t3572 * t2334;
    let t10707 = t2332 * t1289;
    let t10708 = t681 * t10707;
    let t10709 = F::cast_from(4.0_f64) * t10708;
    let t10710 = t37 * t1351;
    let t10712 = F::cast_from(12.0_f64) * t10710 * t2338;
    let t10713 = t189 * t3431;
    let t10714 = t10713 * t581;
    (t10704, t10706, t10709, t10712, t10714)
}
