//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 755/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk755<F: Float>(t35496: F, t1223: F, t211: F, t1965: F, t1977: F, t1982: F, t265: F, t4789: F, t638: F, t71: F, t7311: F, t321: F, t7817: F) -> (F, F, F, F, F) {
    let t35497 = F::cast_from(0.63245127235888530833e-7_f64) * t35496;
    let t35511 = t211 * t1223;
    let t35512 = t1965 * t35511;
    let t35514 = t1977 * t35512 * t1982;
    let t35565 = t638 * t265 * t4789 * t71 * t7311;
    let t35566 = F::cast_from(0.24390119833260022651e-2_f64) * t35565;
    let t35583 = t7817 * t321;
    (t35497, t35512, t35514, t35566, t35583)
}
