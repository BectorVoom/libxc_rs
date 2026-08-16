//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2211/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2211<F: Float>(t25651: F, t3: F, t83120: F, t1409: F, t984: F, t23562: F, t343: F, t1036: F, t25622: F, t14134: F, t6765: F, t1933: F, t23479: F, t88360: F) -> (F, F, F, F, F, F) {
    let t88400 = t83120 * t3 * t25651;
    let t88405 = t1409 * t984;
    let t88407 = t23562 * t88405 * t343;
    let t88415 = t25622 * t1036 / F::cast_from(216.0_f64);
    let t88422 = t6765 * t14134 / F::cast_from(864.0_f64);
    let t88425 = F::cast_from(0.20186378047070195428e-3_f64) * t1933 * t88360 * t23479;
    (t88400, t88405, t88407, t88415, t88422, t88425)
}
