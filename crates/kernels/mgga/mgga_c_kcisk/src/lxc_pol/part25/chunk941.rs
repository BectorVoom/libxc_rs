//! MGGA_C_KCISK lxc pol — lxc_pol part 25 (v4rho3sigma_5) CSE chunk 941/1395 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part25_v4rho3sigma_5_chunk941<F: Float>(t10469: F, t10474: F, t10484: F, t10495: F, t10497: F, t10502: F, t16593: F, t16596: F, t16598: F, t16602: F, t16606: F, t16611: F, t16614: F, t16616: F, t16620: F, t16625: F, t16629: F, t16633: F, t16640: F) -> (F,) {
    let t16642 = -0.66327777777777777776e-2 * t16593 - t16596 - t16598 + 0.11054629629629629629e-2 * t16602 - 0.33163888888888888888e-2 * t16606 - 0.16581944444444444444e-1 * t16611 - t16614 + t16616 - 0.55273148148148148147e-3 * t16620 + 0.66327777777777777776e-2 * t16625 - 0.16581944444444444444e-2 * t16629 - 0.27636574074074074073e-2 * t16633 - 0.22109259259259259258e-2 * t10469 - 0.73697530864197530861e-3 * t10474 - 0.22109259259259259258e-2 * t10484 + 0.18424382716049382715e-2 * t10495 + 0.11054629629629629629e-2 * t10497 + t10502 - 0.55273148148148148147e-3 * t16640;
    (t16642,)
}
