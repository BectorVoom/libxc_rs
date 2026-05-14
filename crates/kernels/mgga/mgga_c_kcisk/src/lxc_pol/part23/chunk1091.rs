//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1091/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1091<F: Float>(t14983: F, t1568: F, t1580: F, t21888: F, t21897: F, t21900: F, t21902: F, t21904: F, t21908: F, t21909: F, t2318: F, t2328: F, t4370: F, t4397: F, t4499: F, t535: F, t541: F, t6482: F, t6583: F) -> (F,) {
    let t21915 = -0.2698618307426597582e-1 * t2318 * t4499 - 0.2698618307426597582e-1 * t535 * t21888 - 0.2698618307426597582e-1 * t4370 * t2328 - 0.5397236614853195164e-1 * t1568 * t6583 + 0.10794473229706390328e0 * t1580 * t21897 - 0.47975436576472845902e-1 * t21900 - 0.59969295720591057378e-2 * t21902 - 0.14392630972941853771e0 * t21904 * t541 + t21908 + 0.26386490117060065246e0 * t21909 * t541 + 0.47975436576472845902e-1 * t14983 - 0.17990788716177317213e-1 * t4397 * t6482;
    (t21915,)
}
