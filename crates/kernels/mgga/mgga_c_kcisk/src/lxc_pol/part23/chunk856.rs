//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 856/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk856<F: Float>(t1417: F, t3561: F, t12825: F, t458: F, t3521: F, t3551: F, t11313: F, t1425: F, t3555: F, t3535: F, t13009: F, t420: F, t1361: F, t3598: F, t1173: F, t3616: F) -> (F, F, F, F, F, F, F, F, F) {
    let t13194 = t1417 * t3561;
    let t13220 = t12825 * t458;
    let t13231 = t3521 * t3551;
    let t13238 = t11313 * t1425;
    let t13240 = t3521 * t3555;
    let t13242 = t3521 * t3535;
    let t13244 = t13009 * t420;
    let t13247 = t3598 * t1361;
    let t13253 = t1173 * t3616;
    (t13194, t13220, t13231, t13238, t13240, t13242, t13244, t13247, t13253)
}
