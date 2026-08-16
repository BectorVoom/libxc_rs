//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 957/1464 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk957<F: Float>(t10640: F, t2508: F, t2541: F, t8682: F, t3437: F, t731: F, t9664: F, t9666: F, t9669: F, t9672: F, t9674: F, t9676: F) -> (F, F, F, F, F) {
    let t10642 = F::cast_from(0.46143157380853345701e-1_f64) * t2508 * t10640;
    let t10643 = t2541 * t8682;
    let t10645 = F::cast_from(0.53833683610995569986e-1_f64) * t2508 * t10643;
    let t10646 = t731 * t3437;
    let t10647 = F::cast_from(0.42725145723012357132e-3_f64) * t10646;
    let t10657 = -F::cast_from(21.0_f64) / F::cast_from(256.0_f64) * t9664 + F::cast_from(147.0_f64) / F::cast_from(8192.0_f64) * t9666 - F::cast_from(63.0_f64) / F::cast_from(524288.0_f64) * t9669 + F::cast_from(21.0_f64) / F::cast_from(524288.0_f64) * t9672 - F::cast_from(49.0_f64) / F::cast_from(8192.0_f64) * t9674 + F::cast_from(7.0_f64) / F::cast_from(256.0_f64) * t9676;
    (t10642, t10643, t10645, t10647, t10657)
}
