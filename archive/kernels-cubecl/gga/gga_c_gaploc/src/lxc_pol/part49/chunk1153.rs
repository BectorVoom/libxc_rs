//! GGA_C_GAPLOC lxc pol — lxc_pol part 49 (v4rhosigma3_14) CSE chunk 1153/1217 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part49_v4rhosigma3_14_chunk1153<F: Float>(t1843: F, t47178: F, t9647: F, t39040: F, t5539: F, t42964: F, t42968: F, t42971: F, t42974: F, t42978: F, t47594: F, t47597: F, t47602: F, t47605: F) -> F {
    let t47607 = t9647 * t1843 * t47178;
    let t47610 = t9647 * t5539 * t39040;
    let t47612 = t42964 + F::cast_from(0.32043859292259267849e-3_f64) * t47594 - t42968 - t42971 - F::cast_from(0.32043859292259267849e-3_f64) * t47597 - t47602 + t47605 - F::cast_from(0.96131577876777803547e-3_f64) * t47607 + F::cast_from(0.64087718584518535698e-3_f64) * t47610 - t42974 - t42978;
    t47612
}
