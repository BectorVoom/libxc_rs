//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 893/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk893<F: Float>(t13361: F, t1411: F, t13318: F, t13323: F, t13325: F, t13334: F, t13338: F, t13342: F, t13344: F, t13347: F, t13351: F, t13355: F, t13359: F) -> (F, F) {
    let t13362 = t1411 * t13361;
    let t13364 = F::new(0.33163888888888888887e-2) * t13318 - F::new(0.49745833333333333332e-2) * t13323 - F::new(0.11054629629629629629e-2) * t13325 - F::new(0.1492375e-1) * t13334 - F::new(0.39796666666666666665e-1) * t13338 + F::new(0.99491666666666666664e-2) * t13342 - F::new(0.66327777777777777775e-2) * t13344 - F::new(0.99491666666666666664e-2) * t13347 - F::new(0.13265555555555555555e-1) * t13351 - F::new(0.22109259259259259258e-1) * t13355 - F::new(0.16581944444444444444e-1) * t13359 - F::new(0.99491666666666666664e-2) * t13362;
    (t13362, t13364)
}
