//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 991/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk991<F: Float>(t1511: F, t492: F, t1414: F, t4232: F, t14234: F, t4231: F, t6368: F, t14556: F, t14558: F, t14561: F, t14563: F, t14565: F, t14568: F, t14571: F, t14575: F, t14579: F, t14582: F, t14584: F, t14586: F, t14589: F) -> (F, F, F) {
    let t14591 = t492 * t1511;
    let t14592 = t1414 * t14591;
    let t14593 = t14592 * t4232;
    let t14595 = t4231 * t14234;
    let t14596 = t6368 * t14595;
    let t14598 = F::cast_from(3.0_f64) / F::cast_from(8.0_f64) * t14556 + F::cast_from(11.0_f64) / F::cast_from(6.0_f64) * t14558 + t14561 / F::cast_from(36.0_f64) - t14563 / F::cast_from(64.0_f64) - t14565 / F::cast_from(2.0_f64) + t14568 / F::cast_from(24.0_f64) + t14571 / F::cast_from(9.0_f64) + t14575 / F::cast_from(54.0_f64) + t14579 / F::cast_from(256.0_f64) - t14582 / F::cast_from(4.0_f64) + t14584 / F::cast_from(16.0_f64) - F::cast_from(19.0_f64) / F::cast_from(48.0_f64) * t14586 - F::cast_from(11.0_f64) / F::cast_from(6.0_f64) * t14589 - t14593 / F::cast_from(6.0_f64) - t14596 / F::cast_from(32.0_f64);
    (t14593, t14596, t14598)
}
