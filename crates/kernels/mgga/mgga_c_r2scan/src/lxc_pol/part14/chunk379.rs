//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 379/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk379<F: Float>(t424: F, t457: F, t41: F, t410: F, t425: F, t1356: F, t1378: F, t1387: F, t1389: F, t1413: F, t1418: F, t1421: F, t1424: F, t1511: F) -> (F, F, F, F, F, F) {
    let t1512 = t424 * t457;
    let t1513 = t41 * t1512;
    let t1514 = F::new(2.0) * t1513;
    let t1515 = t410 * t425;
    let t1516 = F::new(8.0) * t1515;
    let t1517 = -t1387 - t1389 - t1413 + t1418 + t1421 - t1424 + t1511 + t1378 + t1514 - t1516 - t1356;
    (t1512, t1513, t1514, t1515, t1516, t1517)
}
