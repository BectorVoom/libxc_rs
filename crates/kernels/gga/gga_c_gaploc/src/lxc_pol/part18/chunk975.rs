//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 975/1436 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk975<F: Float>(t4130: F, t986: F, t2482: F, t9272: F, t10231: F, t549: F, t544: F, t8410: F, t9562: F, t2365: F, t7906: F, t7025: F) -> (F, F, F, F, F, F, F, F) {
    let t10608 = t4130 * t986;
    let t10609 = t10608 * t2482;
    let t10610 = t9272 * t10609;
    let t10611 = F::cast_from(0.57514388930881124514e0_f64) * t10610;
    let t10612 = t549 * t10231;
    let t10615 = t544 * t8410;
    let t10616 = t10615 * t9562;
    let t10617 = F::cast_from(0.44688112439813033337e-1_f64) * t10616;
    let t10618 = t2365 * t7906;
    let t10619 = t7025 * t10618;
    (t10608, t10609, t10611, t10612, t10615, t10617, t10618, t10619)
}
