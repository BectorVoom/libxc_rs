//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 385/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk385<F: Float>(t1616: F, t783: F, t785: F, t1267: F, t512: F, t507: F, t277: F, t502: F) -> (F, F, F, F) {
    let t1619 = F::new(0.679213007128961539e-1) * t783 * t785 * t1616;
    let t1620 = t512 * t1267;
    let t1622 = F::new(0.29272321618148349056e-1) * t1620 * t507;
    let t1632 = t502 * t277;
    (t1619, t1620, t1622, t1632)
}
