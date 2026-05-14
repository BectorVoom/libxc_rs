//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 1197/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk1197<F: Float>(t32278: F, t8271: F, t1415: F, t8275: F, t1340: F, t8279: F, t8283: F, t34852: F, t34854: F, t34856: F, t34859: F, t34861: F, t34864: F, t34867: F, t34870: F, t34872: F, t34874: F, t34876: F, t34878: F) -> (F, F, F, F, F) {
    let t34880 = t32278 * t8271;
    let t34882 = t1415 * t8275;
    let t34884 = t1340 * t8279;
    let t34886 = t1340 * t8283;
    let t34888 = t34852 / 16.0 - t34854 / 8.0 + t34856 / 12.0 + t34859 / 8.0 - t34861 / 12.0 - t34864 / 16.0 - t34867 / 72.0 + t34870 / 24.0 - t34872 / 128.0 + t34874 / 64.0 - t34876 / 48.0 - t34878 / 64.0 + t34880 / 48.0 + t34882 / 128.0 - t34884 / 288.0 - t34886 / 96.0;
    (t34880, t34882, t34884, t34886, t34888)
}
