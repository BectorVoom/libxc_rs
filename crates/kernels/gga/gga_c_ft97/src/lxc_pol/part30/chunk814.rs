//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 814/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk814<F: Float>(t43548: F, t91: F, t10570: F, t683: F, t24898: F, t56456: F, t10696: F, t1495: F, t263: F, t27742: F, t22511: F, t27519: F, t3789: F, t3758: F, t695: F, t200: F, t668: F) -> (F, F, F, F, F, F, F, F, F) {
    let t99475 = t91 * t43548;
    let t99559 = t683 * t10570;
    let t99672 = t56456 * t24898;
    let t99918 = t1495 * t10696;
    let t107910 = t27742 * t263;
    let t108446 = t27519 * t22511;
    let t108447 = t3789 * t108446;
    let t108517 = t3758 * t695;
    let t108530 = t200 * t668;
    (t99475, t99559, t99672, t99918, t107910, t108446, t108447, t108517, t108530)
}
