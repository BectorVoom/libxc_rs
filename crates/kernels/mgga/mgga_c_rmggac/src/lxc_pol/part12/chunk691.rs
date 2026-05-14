//! MGGA_C_RMGGAC lxc pol — lxc_pol part 12 (v4rho3sigma_3) CSE chunk 691/951 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part12_v4rho3sigma_3_chunk691<F: Float>(t35496: F, t1223: F, t211: F, t1965: F, t1977: F, t1982: F, t2004: F, t7939: F, t118: F, t1986: F, t338: F, t352: F, t495: F, t1257: F, t1995: F, t326: F, t333: F) -> (F, F, F, F, F, F, F) {
    let t35497 = 0.63245127235888530833e-7 * t35496;
    let t35511 = t211 * t1223;
    let t35512 = t1965 * t35511;
    let t35514 = t1977 * t35512 * t1982;
    let t35516 = t7939 * t2004;
    let t35523 = t1986 * t118 * t338 * t495 * t352;
    let t35535 = t1986 * t1257;
    let t35551 = t1986 * t326 * t1995 * t333;
    (t35497, t35512, t35514, t35516, t35523, t35535, t35551)
}
