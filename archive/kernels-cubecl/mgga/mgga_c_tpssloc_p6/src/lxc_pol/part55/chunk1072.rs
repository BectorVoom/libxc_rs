//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1072/1304 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1072<F: Float>(t1218: F, t1232: F, t2134: F, t32425: F, t32429: F, t32433: F, t32436: F, t32441: F, t32445: F, t32448: F, t488: F, t7316: F, t7326: F, t8875: F) -> F {
    let t32451 = t32425 - F::cast_from(0.40372756094140390856e-3_f64) * t7316 * t8875 - F::cast_from(0.40372756094140390856e-3_f64) * t2134 * t32429 + F::cast_from(0.40372756094140390856e-3_f64) * t7326 * t32433 + t32436 * t488 / F::cast_from(1536.0_f64) + t32441 * t1218 / F::cast_from(1536.0_f64) + t32445 - t32448 * t1232 / F::cast_from(2304.0_f64);
    t32451
}
