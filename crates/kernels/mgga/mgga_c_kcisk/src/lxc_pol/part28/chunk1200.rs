//! MGGA_C_KCISK lxc pol — lxc_pol part 28 (v4rho3sigma_8) CSE chunk 1200/1456 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part28_v4rho3sigma_8_chunk1200<F: Float>(t34389: F, t9740: F, t6714: F, t9741: F, t7246: F, t2023: F, t2647: F, t33197: F, t7261: F) -> (F, F, F, F, F, F) {
    let t34390 = t9740 * t34389;
    let t34394 = t9741 * t6714;
    let t34395 = t7246 * t34394;
    let t34398 = t2647 * t2023;
    let t34399 = t33197 * t34398;
    let t34400 = t7261 * t34399;
    (t34390, t34394, t34395, t34398, t34399, t34400)
}
