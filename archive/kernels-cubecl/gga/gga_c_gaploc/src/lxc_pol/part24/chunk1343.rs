//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 1343/1439 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk1343<F: Float>(t1382: F, t8435: F, t921: F, t2902: F, t6553: F, t24295: F, t2595: F, t11135: F, t5559: F, t841: F, t24282: F, t7324: F, t8859: F) -> (F, F, F, F, F, F) {
    let t33966 = F::cast_from(2.0_f64) * t1382 * t8435 * t921;
    let t33968 = F::cast_from(2.0_f64) * t6553 * t2902;
    let t33970 = F::cast_from(4.0_f64) * t24295 * t2595;
    let t33973 = F::cast_from(12.0_f64) * t5559 * t11135 * t841;
    let t33974 = t24282 * t921;
    let t33977 = F::cast_from(4.0_f64) * t7324 * t8859;
    (t33966, t33968, t33970, t33973, t33974, t33977)
}
