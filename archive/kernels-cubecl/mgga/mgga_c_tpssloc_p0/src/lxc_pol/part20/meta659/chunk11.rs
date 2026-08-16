//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2463/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2463<F: Float>(t3185: F, t49649: F, t11031: F, t11054: F, t11081: F, t14578: F, t14596: F, t14605: F, t14608: F, t14622: F, t1629: F, t1630: F, t3076: F, t3131: F, t3180: F, t3186: F, t3189: F, t3200: F, t43473: F, t43515: F, t43542: F, t4669: F, t4680: F, t4684: F, t4691: F, t47819: F) -> F {
    let t50465 = t49649 * t3185;
    let t50490 = F::cast_from(14.0_f64) * t1629 * t3131 * t43515 * t47819 + F::cast_from(6.0_f64) * t11054 * t3186 * t4680 - F::cast_from(3.0_f64) * t14605 * t3200 * t4684 - F::cast_from(3.0_f64) * t14622 * t3200 * t4680 + F::cast_from(3.0_f64) * t11031 * t4669 - F::cast_from(3.0_f64) * t11081 * t14608 + F::cast_from(18.0_f64) * t14578 * t43473 + F::cast_from(3.0_f64) * t14596 * t3180 + t1630 * t43542 + F::cast_from(3.0_f64) * t3076 * t4691 + F::cast_from(6.0_f64) * t3189 * t50465;
    t50490
}
