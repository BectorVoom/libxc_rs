//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 966/1200 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk966<F: Float>(t31805: F, t32237: F, t32240: F, t1419: F, t8477: F, t1385: F, t9656: F, t1444: F, t8578: F, t3999: F, t4075: F, t1398: F, t543: F) -> (F, F, F, F, F, F, F) {
    let t32244 = t31805 * t32237;
    let t32246 = F::cast_from(0.25389723392137995738e-1_f64) * t32244 * t32240;
    let t32247 = t8477 * t1419;
    let t32250 = t9656 * t1385;
    let t32252 = t32250 * t8578 * t1444;
    let t32255 = t4075 * t3999;
    let t32257 = t8578 * t1398 * t543;
    (t32244, t32246, t32247, t32250, t32252, t32255, t32257)
}
