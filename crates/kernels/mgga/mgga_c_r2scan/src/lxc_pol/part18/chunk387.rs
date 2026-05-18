//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 387/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk387<F: Float>(t1654: F, t170: F, t188: F, t631: F, t189: F, t621: F, t390: F, t649: F, t652: F, t124: F, t4: F, t615: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t1655 = t1654 * t170;
    let t1658 = t631 * t188;
    let t1659 = t189 * t621;
    let t1660 = t1658 * t1659;
    let t1662 = F::new(0.71233333333333333332e-1) * t390 * t1660;
    let t1663 = t649 * t188;
    let t1664 = t652 * t621;
    let t1665 = t1663 * t1664;
    let t1667 = F::new(0.57278650314509912396e0) * t390 * t1665;
    let t1668 = t4 * t124;
    let t1669 = t615 * t1668;
    (t1655, t1658, t1659, t1660, t1662, t1663, t1664, t1665, t1667, t1668, t1669)
}
