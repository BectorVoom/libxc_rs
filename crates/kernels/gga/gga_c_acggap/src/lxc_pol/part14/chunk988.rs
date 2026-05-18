//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 988/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk988<F: Float>(t34957: F, t1181: F, t21143: F, t604: F, t7493: F, t1992: F, t5606: F, t7585: F, t7586: F, t1432: F, t30147: F, t1494: F, t7329: F) -> (F, F, F, F, F) {
    let t34958 = F::new(0.28582678745379824648e-3) * t34957;
    let t34961 = t7493 * t1181 * t604 * t21143;
    let t34962 = F::new(0.31448092289604152068e-2) * t34961;
    let t34990 = t7585 * t7586 * t1992 * t5606;
    let t34991 = F::new(0.28582678745379824648e-3) * t34990;
    let t35022 = t30147 * t7586 * t1992 * t1432;
    let t35039 = t7329 * t1494;
    (t34958, t34962, t34991, t35022, t35039)
}
