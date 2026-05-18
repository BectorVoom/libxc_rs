//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 720/1072 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk720<F: Float>(t12670: F, t2610: F, t3720: F, t2365: F, t2033: F, t12252: F, t959: F, t13861: F, t1457: F, t2103: F, t12256: F, t3470: F) -> (F, F, F, F, F, F, F, F) {
    let t13890 = F::new(0.38342925953920749677e0) * t12670;
    let t13891 = t2610 * t3720;
    let t13892 = t2365 * t13891;
    let t13893 = t2033 * t13892;
    let t13895 = t12252 * t959;
    let t13900 = t1457 * t13861;
    let t13901 = t2103 * t13900;
    let t13904 = t12256 * t3470;
    (t13890, t13891, t13892, t13893, t13895, t13900, t13901, t13904)
}
