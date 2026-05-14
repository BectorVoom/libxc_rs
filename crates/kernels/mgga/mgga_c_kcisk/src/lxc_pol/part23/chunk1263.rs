//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1263/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1263<F: Float>(t3507: F, t394: F, t32366: F, t9532: F, t32379: F, t4419: F, t2737: F, t32377: F, t32370: F, t9515: F, t25: F, t32457: F, t32460: F, t9536: F, t1310: F, t1311: F, t4374: F) -> (F, F, F, F, F, F, F, F, F) {
    let t109420 = t3507 * t394;
    let t109448 = t32366 * t9532;
    let t109460 = t4419 * t32379;
    let t109461 = t2737 * t109460;
    let t109487 = t32377 * t109460;
    let t109489 = t9515 * t32370;
    let t109494 = t25 * t32457;
    let t109496 = t9536 * t109494 * t32460;
    let t109499 = t1310 * t1311 * t4374;
    (t109420, t109448, t109460, t109461, t109487, t109489, t109494, t109496, t109499)
}
