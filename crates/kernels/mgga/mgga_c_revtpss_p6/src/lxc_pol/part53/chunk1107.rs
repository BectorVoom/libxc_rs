//! MGGA_C_REVTPSS lxc pol — lxc_pol part 53 (v4rho2sigma2_8) CSE chunk 1107/1244 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part53_v4rho2sigma2_8_chunk1107<F: Float>(t121034: F, t1385: F, t1404: F, t32276: F, t32278: F, t3985: F, t8591: F, t240: F, t843: F, t31752: F, t32197: F, t136: F, t2457: F, t545: F) -> (F, F, F, F, F, F) {
    let t121035 = t121034 * t1385;
    let t121043 = t32276 * t1404 * t32278;
    let t121045 = t8591 * t3985;
    let t121046 = F::cast_from(0.49169913065300780973e-2_f64) * t121045;
    let t121056 = t1385 * t843 * t240;
    let t121057 = t31752 * t121056;
    let t121058 = t121057 * t32197;
    let t121072 = t545 * t136 * t2457;
    (t121035, t121043, t121046, t121057, t121058, t121072)
}
