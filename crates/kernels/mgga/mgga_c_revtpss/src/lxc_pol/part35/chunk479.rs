//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 479/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk479<F: Float>(t2453: F, t556: F, t136: F, t561: F, t2457: F, t1426: F, t786: F, t1363: F, t2470: F, t1362: F, t1386: F, t820: F, t843: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3906 = t2453 * t556;
    let t3907 = t561 * t136;
    let t3908 = t3907 * t2457;
    let t3910 = F::new(0.11565819519348392139e-2) * t3906 * t3908;
    let t3914 = t556 * t1426;
    let t3915 = t786 * t3914;
    let t3920 = t1363 * t2470;
    let t3922 = F::new(0.13009920719177044025e-1) * t1362 * t3920;
    let t3930 = t820 * t1386 * t843;
    (t3906, t3907, t3908, t3910, t3914, t3915, t3920, t3922, t3930)
}
