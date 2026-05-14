//! MGGA_C_REVTPSS lxc pol — lxc_pol part 51 (v4rho2sigma2_6) CSE chunk 942/1050 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part51_v4rho2sigma2_6_chunk942<F: Float>(t3985: F, t8591: F, t1385: F, t240: F, t843: F, t31752: F, t32197: F, t136: F, t2457: F, t545: F, t25304: F, t32217: F, t8477: F, t8705: F, t9656: F, t3999: F, t8578: F) -> (F, F, F, F, F, F, F) {
    let t121045 = t8591 * t3985;
    let t121046 = 0.49169913065300780973e-2 * t121045;
    let t121056 = t1385 * t843 * t240;
    let t121057 = t31752 * t121056;
    let t121058 = t121057 * t32197;
    let t121072 = t545 * t136 * t2457;
    let t121074 = 0.45699670022203476294e-2 * t25304 * t32217 * t121072;
    let t121076 = t8477 * t8705 * t9656;
    let t121077 = t3999 * t8578;
    (t121046, t121057, t121058, t121072, t121074, t121076, t121077)
}
