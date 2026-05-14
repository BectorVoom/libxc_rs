//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 795/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk795<F: Float>(t2283: F, t38638: F, t1737: F, t2084: F, t27: F, t7273: F, t3351: F, t511: F, t6382: F, t9188: F, t3352: F, t6387: F, t880: F, t2144: F, t6530: F, t1929: F, t1986: F) -> (F, F, F, F, F, F) {
    let t44977 = t38638 * t2283;
    let t44982 = t7273 * t27 * t2084 * t1737;
    let t44986 = t3351 * t9188 * t511 * t6382;
    let t44990 = t3351 * t3352 * t880 * t6387;
    let t44994 = t3351 * t3352 * t2144 * t6530;
    let t44996 = t1986 * t1929;
    (t44977, t44982, t44986, t44990, t44994, t44996)
}
