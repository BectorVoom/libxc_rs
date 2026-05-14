//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 670/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk670<F: Float>(t24: F, t3951: F, t398: F, t963: F, t13522: F, t344: F, t3118: F, t313: F, t353: F, t347: F, t355: F, t1232: F, t4079: F, t346: F, t360: F, t4082: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t13607 = t24 * t3951;
    let t13614 = t963 * t398;
    let t13618 = 28.0 / 27.0 * t13522;
    let t13632 = 1.0/pow_3_2(t344);
    let t13665 = t353 * t3118 * t313;
    let t13666 = 0.73028148148148148147e0 * t13665;
    let t13669 = 1.0 / t347 / t355 / 8.0;
    let t13672 = 0.93011851851851851854e0 * t13522;
    let t13679 = 1.0 / t4079 / t1232;
    let t13680 = t346 * t13679;
    let t13682 = 1.0 / t4082 / t360;
    (t13607, t13614, t13618, t13632, t13665, t13666, t13669, t13672, t13680, t13682)
}
