//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 1040/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk1040<F: Float>(t27501: F, t33404: F, t27633: F, t140869: F, t33366: F, t6804: F, t6762: F, t695: F, t1541: F, t2404: F, t33435: F, t3746: F) -> (F, F, F, F, F) {
    let t150876 = t33404 * t27501;
    let t150879 = t33404 * t27633;
    let t150883 = t33366 * t140869 * t6804;
    let t150887 = t6762 * t695;
    let t150902 = t33435 * t1541 * t2404 * t3746;
    (t150876, t150879, t150883, t150887, t150902)
}
