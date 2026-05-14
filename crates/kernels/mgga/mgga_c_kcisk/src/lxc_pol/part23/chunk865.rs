//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 865/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk865<F: Float>(t1311: F, t163: F, t1248: F, t3575: F, t24: F, t3951: F, t398: F, t963: F, t1163: F, t13522: F, t344: F, t3583: F, t3979: F, t3118: F, t313: F, t353: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13603 = t163 * t1311;
    let t13605 = t1248 * t13603 * t3575;
    let t13607 = t24 * t3951;
    let t13614 = t963 * t398;
    let t13616 = t1248 * t13614 * t1163;
    let t13618 = 28.0 / 27.0 * t13522;
    let t13632 = 1.0/pow_3_2(t344);
    let t13650 = t1248 * t3979 * t3583;
    let t13665 = t353 * t3118 * t313;
    (t13603, t13605, t13607, t13614, t13616, t13618, t13632, t13650, t13665)
}
