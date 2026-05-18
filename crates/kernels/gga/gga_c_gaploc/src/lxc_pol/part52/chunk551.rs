//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 551/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk551<F: Float>(t10615: F, t9562: F, t2365: F, t7906: F, t7025: F, t1022: F, t935: F) -> (F, F, F, F, F) {
    let t10616 = t10615 * t9562;
    let t10617 = F::new(0.44688112439813033337e-1) * t10616;
    let t10618 = t2365 * t7906;
    let t10619 = t7025 * t10618;
    let t10620 = F::new(0.14896037479937677779e-1) * t10619;
    let t10627 = t1022 * t935;
    (t10616, t10617, t10619, t10620, t10627)
}
