//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 329/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk329<F: Float>(t227: F, t1456: F, t1521: F, t1607: F, t1611: F, t1620: F, t240: F, t555: F, t297: F, t1060: F, t565: F, t298: F, t430: F, t569: F, zeta_threshold: F) -> (F, F, F, F) {
    let t228 = t227 <= zeta_threshold;
    let t1624 = t1456 - t1521 + t240 * (t1607 * t555 - t1611 * t1620 - t1456 + t1521);
    let t1625 = t297 * t1624;
    let t1628 = piecewise3::<f64>(t228, F::new(0.0), t1060);
    let t1629 = t565 * t1628;
    let t1634 = t298 * t430 * t569;
    (t1624, t1625, t1629, t1634)
}
