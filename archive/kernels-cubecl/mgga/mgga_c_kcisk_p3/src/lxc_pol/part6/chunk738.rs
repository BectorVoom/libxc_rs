//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 738/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk738<F: Float>(t12825: F, t41: F, t12829: F, t451: F, t13329: F, t492: F, t1555: F, t524: F, t4349: F, t544: F, t13399: F, t13064: F, t325: F) -> (F, F, F, F, F, F, F) {
    let t14496 = t41 * t12825;
    let t14497 = t451 * t12829;
    let t14545 = t13329 * t492;
    let t14607 = t1555 * t1555;
    let t14608 = F::cast_from(1.0_f64) / t14607;
    let t14609 = t524 * t14608;
    let t14612 = F::cast_from(1.0_f64) / t4349 / t544;
    let t14665 = F::cast_from(0.51588271604938271604e-3_f64) * t13399;
    let t14736 = t325 * t13064;
    (t14496, t14497, t14545, t14609, t14612, t14665, t14736)
}
