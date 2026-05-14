//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 444/1124 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk444<F: Float>(t1421: F, t1424: F, t1459: F, t1463: F, t1470: F, t1480: F, t1488: F, t1511: F, t1514: F, t1516: F, t1519: F, t1522: F, t1526: F, t1529: F, t1533: F, t11: F, t1643: F, t1645: F, t2040: F, t2073: F, t5: F) -> (F,) {
    let t2074 = -t1421 + t1424 - t1511 - t1519 + t1459 - t1526 - t1514 + t1516 + t1470 - t1480 - t1488 - t1529 + t1463 + t1522 - t1533;
    let t2078 = t1643 - 10.0 / 3.0 * t1645 + 5.0 * t5 * t11 * t2040 - 45.0 * param_eta * (t2073 + t2074);
    (t2078,)
}
