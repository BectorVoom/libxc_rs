//! MGGA_C_RMGGAC lxc pol — lxc_pol part 16 (v4rho3sigma_7) CSE chunk 902/1158 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part16_v4rho3sigma_7_chunk902<F: Float>(t3351: F, t3352: F, t6387: F, t880: F, t2144: F, t6530: F, t1929: F, t1986: F, t7720: F, t495: F, t515: F, t6522: F, t7230: F) -> (F, F, F, F) {
    let t44990 = t3351 * t3352 * t880 * t6387;
    let t44994 = t3351 * t3352 * t2144 * t6530;
    let t44996 = t1986 * t1929;
    let t44997 = t7720 * t44996;
    let t45002 = t7230 * t3352 * t515 * t6522 * t495;
    (t44990, t44994, t44997, t45002)
}
