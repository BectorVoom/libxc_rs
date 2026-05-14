//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1394/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1394<F: Float>(t1339: F, t32203: F, t6229: F, t19058: F, t9461: F, t19062: F, t3759: F, t109420: F, t6225: F, t33482: F, t9442: F, t1333: F, t33561: F, t110081: F, t110463: F, t110566: F, t110762: F, t114199: F, t114205: F, t1220: F, t20: F, t2158: F, t2718: F, t32127: F, t32131: F, t33384: F, t33389: F, t3913: F, t9449: F) -> (F, F, F, F, F, F) {
    let t114755 = t1339 * t32203 * t6229;
    let t114758 = t1339 * t9461 * t19058;
    let t114761 = t3759 * t9461 * t19062;
    let t114764 = t1339 * t109420 * t6225;
    let t114773 = 0.18518518518518518519e-1 * t33482 * t9442;
    let t114774 = t1333 * t33561;
    let t114776 = -0.34722222222222222223e-2 * t33384 * t32127 - 0.46296296296296296297e-2 * t33384 * t32131 - 0.46561250000000000002e-2 * t110566 * t33389 - 0.46561250000000000002e-2 * t110463 * t33389 - 0.69444444444444444446e-2 * t114199 * t9449 - 0.69444444444444444446e-2 * t114205 * t9449 + 0.22109259259259259258e-2 * t114755 + 0.11054629629629629629e-2 * t114758 + 0.18424382716049382715e-2 * t114761 - 0.33163888888888888888e-2 * t114764 + t110762 - 0.16581944444444444444e-2 * t110081 - 0.10185185185185185186e0 * t1220 * t2158 * t3913 * t20 * t2718 + t114773 - 0.88437037037037037034e-2 * t114774;
    (t114755, t114758, t114761, t114764, t114774, t114776)
}
