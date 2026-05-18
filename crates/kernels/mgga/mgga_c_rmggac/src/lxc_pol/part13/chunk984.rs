//! MGGA_C_RMGGAC lxc pol — lxc_pol part 13 (v4rho3sigma_4) CSE chunk 984/1127 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part13_v4rho3sigma_4_chunk984<F: Float>(t2010: F, t2012: F, t5002: F, t2019: F, t2020: F, t8850: F, t1652: F, t1971: F, t495: F, t515: F, t7230: F, t34944: F, t40888: F) -> (F, F, F, F) {
    let t41616 = t2010 * t2012 * t5002;
    let t41619 = t2019 * t2020 * t8850;
    let t41627 = t7230 * t1971 * t515 * t1652 * t495;
    let t41631 = t34944 * t40888;
    (t41616, t41619, t41627, t41631)
}
