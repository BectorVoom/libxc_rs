//! HYB_MGGA_XC_GAS22 lxc pol — lxc_pol part 6 (v4rho4_2) CSE chunk 949/1455 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI, M_SQRT2};
use libxc_kernel_math::erf::{erf_approx};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn hyb_mgga_xc_gas22_lxc_pol_part6_v4rho4_2_chunk949<F: Float>(t8651: F, t2194: F, t3329: F, t791: F, t2200: F, t3324: F, t2206: F, t3335: F, t6530: F, t6533: F, t6616: F, t6619: F, t6622: F, t6655: F, t8648: F) -> (F, F, F, F, F, F) {
    let t8652 = F::cast_from(0.60385e0_f64) * t8651;
    let t8653 = t2194 * t3329;
    let t8654 = t8653 * t791;
    let t8656 = t3324 * t2200;
    let t8658 = t2206 * t3329;
    let t8659 = t8658 * t791;
    let t8661 = t3335 * t2200;
    let t8668 = F::cast_from(0.905775e0_f64) * t8648 - t8652 - F::cast_from(0.258925e1_f64) * t8654 - F::cast_from(0.1294625e1_f64) * t8656 + F::cast_from(0.16504875e0_f64) * t8659 + F::cast_from(0.82524375e-1_f64) * t8661 + F::cast_from(0.80513333333333333334e0_f64) * t6530 - F::cast_from(0.301925e0_f64) * t6533 - t6655 + F::cast_from(0.5519e0_f64) * t6616 - F::cast_from(0.16557e0_f64) * t6619 - F::cast_from(0.16557e0_f64) * t6622;
    (t8652, t8654, t8656, t8659, t8661, t8668)
}
