//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 795/938 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk795<F: Float>(t12951: F, t459: F, t12830: F, t3530: F, t11313: F, t1425: F, t3521: F, t3555: F, t3535: F, t13009: F, t420: F, t1361: F, t3598: F, t1175: F, t3587: F, t1173: F, t3616: F) -> (F, F, F, F, F, F, F, F) {
    let t13233 = t459 * t12951;
    let t13235 = t3530 * t13233 * t12830;
    let t13238 = t11313 * t1425;
    let t13240 = t3521 * t3555;
    let t13242 = t3521 * t3535;
    let t13244 = t13009 * t420;
    let t13247 = t3598 * t1361;
    let t13250 = t1175 * t3587;
    let t13253 = t1173 * t3616;
    (t13235, t13238, t13240, t13242, t13244, t13247, t13250, t13253)
}
