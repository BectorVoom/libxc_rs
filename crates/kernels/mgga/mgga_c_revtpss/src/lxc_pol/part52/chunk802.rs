//! MGGA_C_REVTPSS lxc pol — lxc_pol part 52 (v4rho2sigma2_7) CSE chunk 802/1144 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part52_v4rho2sigma2_7_chunk802<F: Float>(t25299: F, t26519: F, t25365: F, t7407: F, t25310: F, t25305: F, t26506: F, t7058: F, t2471: F, t7388: F, t25375: F, t26485: F, t72: F, t7423: F, t686: F, t213: F, t7398: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t26521 = 0.17135234354032049604e-2 * t25299 * t26519;
    let t26522 = t25365 * t7407;
    let t26529 = t25310 * t7407;
    let t26534 = 0.22849835011101738147e-2 * t25305 * t26519;
    let t26536 = 0.96373646535613327357e-2 * t7058 * t26506;
    let t26538 = 0.13009920719177044025e-1 * t7388 * t2471;
    let t26541 = t25375 * t26485;
    let t26543 = t7423 * t72;
    let t26544 = t26543 * t686;
    let t26545 = t7058 * t26544;
    let t26547 = t213 * t7398;
    (t26521, t26522, t26529, t26534, t26536, t26538, t26541, t26544, t26545, t26547)
}
