//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 257/1243 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk257<F: Float>(t1307: F, t1451: F, t104: F, t111: F, t120: F, t1404: F, t1424: F, t1429: F, t1431: F, t1436: F, t1438: F, t1442: F, t1445: F, t1450: F, t833: F) -> (F, F) {
    let t1452 = t1451 * t1307;
    let t1455 = t1424 + 0.11955719325063177623e-1 * t1404 * t833 - t1429 - 0.3513e-2 * t104 * t1431 + t1436 + 0.7925e-3 * t111 * t1438 - t1442 - 0.5179538907796306876e-4 * t1445 * t833 + t1450 + 0.50413125e-5 * t120 * t1452;
    (t1452, t1455)
}
