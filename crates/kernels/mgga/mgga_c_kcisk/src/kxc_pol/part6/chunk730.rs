//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 730/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk730<F: Float>(t373: F, t4128: F, t357: F, t4079: F, t346: F, t1311: F, t163: F, t24: F, t3951: F, t398: F, t963: F, t13522: F) -> (F, F, F, F, F, F) {
    let t13565 = F::cast_from(1.0_f64) / t4128 / t373;
    let t13587 = F::cast_from(1.0_f64) / t4079 / t357;
    let t13588 = t346 * t13587;
    let t13603 = t163 * t1311;
    let t13607 = t24 * t3951;
    let t13614 = t963 * t398;
    let t13618 = F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t13522;
    (t13565, t13588, t13603, t13607, t13614, t13618)
}
