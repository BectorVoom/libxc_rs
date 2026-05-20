//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 1329/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk1329<F: Float>(t1937: F, t75941: F, t114373: F, t18245: F, t7735: F, t22852: F, t28167: F, t8996: F, t29506: F, t7901: F, t1907: F, t6836: F) -> (F, F, F, F, F, F) {
    let t114438 = F::new(2.0) * t75941 * t1937;
    let t114440 = F::new(6.0) * t114373 * t1937;
    let t114442 = F::new(6.0) * t18245 * t7735;
    let t114445 = F::new(18.0) * t28167 * t8996 * t22852;
    let t114451 = F::new(9.0) * t29506 * t7901;
    let t114452 = t6836 * t1907;
    (t114438, t114440, t114442, t114445, t114451, t114452)
}
