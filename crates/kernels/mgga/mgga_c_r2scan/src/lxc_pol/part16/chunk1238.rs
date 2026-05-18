//! MGGA_C_R2SCAN lxc pol — lxc_pol part 16 (v4rho3sigma_6) CSE chunk 1238/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part16_v4rho3sigma_6_chunk1238<F: Float>(t11476: F, t40282: F, t11560: F, t40713: F, t42846: F, t481: F, t37327: F, t4176: F, t35373: F, t795: F, t14160: F, t40630: F) -> (F, F, F, F) {
    let t43764 = F::new(3.0) / F::new(2.0) * t40282 * t11476;
    let t43766 = F::new(5.0) / F::new(8.0) * t40713 * t11560;
    let t43767 = t42846 * t481;
    let t43770 = F::new(15.0) / F::new(8.0) * t37327 * t4176 * t43767;
    let t43771 = t35373 * t795;
    let t43774 = F::new(3.0) * t40630 * t14160 * t43771;
    (t43764, t43766, t43770, t43774)
}
