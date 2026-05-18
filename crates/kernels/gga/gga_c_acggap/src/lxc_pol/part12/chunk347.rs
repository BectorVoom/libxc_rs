//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 347/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk347<F: Float>(t906: F, t1380: F, t1383: F, t1385: F, t1387: F, t1389: F, t1391: F, t1392: F, t764: F, t774: F, t782: F, t905: F, t914: F) -> F {
    let t1393 = F::new(4.0) * t906;
    let t1394 = t1380 - t1383 + t1385 - t1387 + t1389 + t1391 + t914 - t1392 - t905 - t1393 - t764 + t774 + t782;
    t1394
}
