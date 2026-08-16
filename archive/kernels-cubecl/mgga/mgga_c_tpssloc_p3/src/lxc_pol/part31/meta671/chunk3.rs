//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2004/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2004<F: Float>(t55921: F, t7025: F, t2240: F, t5392: F, t63: F, t2032: F, t26067: F, t26911: F, t27966: F, t28935: F, t6492: F, t6495: F, t7026: F, t7035: F, t91959: F, t96393: F, t96406: F, t96479: F, t96482: F, t96517: F, t96521: F) -> F {
    let t102267 = t55921 * t7025;
    let t102275 = t2240 * t5392 * t63;
    let t102278 = -F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t26911 * t26067 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t96406 * t2032 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t96479 * t2032 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t96482 * t2032 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t27966 * t7035 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7026 * t96393 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6495 * t28935 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t102267 * t6492 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7026 * t96517 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7026 * t96521 + F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t102275 * t6492 + t91959;
    t102278
}
