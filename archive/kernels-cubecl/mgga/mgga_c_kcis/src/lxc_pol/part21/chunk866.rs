//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 866/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk866<F: Float>(t13330: F, t9517: F, t3200: F, t3178: F, t4985: F, t1092: F, t2825: F, t4814: F, t3182: F, t4984: F, t1096: F, t1662: F, t9476: F) -> (F, F, F, F, F) {
    let t13331 = t9517 * t13330;
    let t13332 = t3200 * t13331;
    let t13336 = t3178 * t4985;
    let t13337 = t1092 * t13336;
    let t13339 = t2825 * t4814;
    let t13340 = t1092 * t13339;
    let t13342 = t3182 * t4984;
    let t13343 = t1096 * t13342;
    let t13344 = t1092 * t13343;
    let t13346 = t9476 * t1662;
    (t13332, t13337, t13340, t13344, t13346)
}
