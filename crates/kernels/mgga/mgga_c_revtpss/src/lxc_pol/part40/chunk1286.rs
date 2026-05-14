//! MGGA_C_REVTPSS lxc pol — lxc_pol part 40 (v4rho3tau_3) CSE chunk 1286/1348 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part40_v4rho3tau_3_chunk1286<F: Float>(t15936: F, t17550: F, t1042: F, t3708: F, t5265: F, t13392: F, t5302: F, t1252: F, t1261: F, t12956: F, t17525: F, t17529: F, t17536: F, t17541: F, t17546: F, t17547: F, t3591: F, t3606: F, t3613: F, t3711: F, t5293: F, t5299: F) -> (F,) {
    let t17551 = t17550 * t15936;
    let t17552 = t1042 * t17551;
    let t17556 = 0.28582678745379824648e-3 * t3708 * t5265;
    let t17557 = t5302 * t13392;
    let t17558 = t1042 * t17557;
    let t17561 = -0.11433071498151929859e-2 * t5293 * t3591 - 0.22866142996303859718e-2 * t17525 * t3606 + 0.11433071498151929859e-2 * t17529 * t3613 + 0.28582678745379824648e-3 * t12956 * t5299 + 0.28582678745379824648e-3 * t3711 * t17536 + 0.14291339372689912324e-3 * t3711 * t17541 + t17546 - 0.22866142996303859718e-2 * t17547 * t1252 + 0.14291339372689912324e-2 * t1261 * t17552 + t17556 + 0.23818898954483187207e-3 * t1261 * t17558;
    (t17561,)
}
