//! MGGA_C_REVTPSS lxc pol — lxc_pol part 27 (v4rho3sigma_2) CSE chunk 1330/1333 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part27_v4rho3sigma_2_chunk1330<F: Float>(t10194: F, t1310: F, t1453: F, t2163: F, t2328: F, t26804: F, t27066: F, t27079: F, t4151: F, t4254: F, t508: F, t7683: F, t7687: F, t95066: F, t95068: F, t95070: F, t95073: F, t95075: F, t95081: F, t95083: F, t95085: F, t95087: F, t95090: F, t95096: F, t95104: F, t95108: F, t96709: F) -> F {
    let t97565 = -F::new(6.0) * t10194 * t2163 - F::new(6.0) * t1310 * t26804 + F::new(3.0) * t1453 * t27066 - F::new(6.0) * t2328 * t7683 - F::new(6.0) * t27079 * t4254 + F::new(3.0) * t4151 * t7687 - F::new(6.0) * t508 * t96709 - t95066 - t95068 - t95070 - t95073 - t95075 + t95081 - t95083 - t95085 - t95087 - t95090 + t95096 - t95104 + t95108;
    t97565
}
