//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 748/880 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk748<F: Float>(t11780: F, t2679: F, t9805: F, t23477: F, t44708: F, t4820: F, t43432: F, t2617: F, t3630: F, t7803: F, t11894: F, t1445: F, t2087: F, t2530: F, t11801: F, t41105: F) -> (F, F, F, F, F, F) {
    let t45238 = t9805 * t11780 * t2679;
    let t45242 = 0.23833659967900284446e0 * t23477 * t4820 * t44708;
    let t45243 = 0.11916829983950142223e0 * t43432;
    let t45246 = t7803 * t3630 * t2617;
    let t45247 = 0.19171462976960374838e0 * t45246;
    let t45251 = 0.69017266717057349418e1 * t2087 * t1445 * t11894 * t2530;
    let t45256 = 0.42900587942220512003e1 * t11801 * t41105;
    (t45238, t45242, t45243, t45247, t45251, t45256)
}
