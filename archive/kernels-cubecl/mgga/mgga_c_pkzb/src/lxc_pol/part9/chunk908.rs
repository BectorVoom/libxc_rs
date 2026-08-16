//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 908/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk908<F: Float>(t12: F, t24: F, t1064: F, t1429: F, t1643: F, t1646: F, t207: F, t2562: F, t2732: F, t2735: F, t6771: F, t82: F, t1165: F, t1652: F, t1655: F, t2569: F, t3019: F, t3022: F, t333: F, t6786: F, zeta_threshold: F) -> (F, F) {
    let t84 = t12 <= zeta_threshold;
    let t90 = t24 <= zeta_threshold;
    let t6839 = piecewise3::<F>(t84, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t2732 * t1643 - F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t2735 * t6771 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1064 * t1646 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t207 * t1429 - F::cast_from(4.0_f64) * t2562 * t82);
    let t6851 = piecewise3::<F>(t90, F::cast_from(0.0_f64), F::cast_from(8.0_f64) / F::cast_from(27.0_f64) * t3019 * t1652 + F::cast_from(8.0_f64) / F::cast_from(9.0_f64) * t3022 * t6786 - F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t1165 * t1655 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t333 * t1429 + F::cast_from(4.0_f64) * t2569 * t82);
    (t6839, t6851)
}
