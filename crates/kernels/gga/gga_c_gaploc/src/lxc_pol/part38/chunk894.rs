//! GGA_C_GAPLOC lxc pol — lxc_pol part 38 (v4rhosigma3_3) CSE chunk 894/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part38_v4rhosigma3_3_chunk894<F: Float>(t23477: F, t44708: F, t4820: F, t43432: F, t2617: F, t3630: F, t7803: F, t11894: F, t1445: F, t2087: F, t2530: F, t313: F, t45001: F) -> (F, F, F, F, F) {
    let t45242 = F::new(0.23833659967900284446e0) * t23477 * t4820 * t44708;
    let t45243 = F::new(0.11916829983950142223e0) * t43432;
    let t45246 = t7803 * t3630 * t2617;
    let t45247 = F::new(0.19171462976960374838e0) * t45246;
    let t45251 = F::new(0.69017266717057349418e1) * t2087 * t1445 * t11894 * t2530;
    let t45252 = t313 * t45001;
    (t45242, t45243, t45247, t45251, t45252)
}
