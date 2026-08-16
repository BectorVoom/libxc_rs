//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2280/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2280<F: Float>(t24996: F, t90442: F, t24995: F, t34475: F, t5308: F, t1983: F, t26503: F, t6999: F, t12823: F, t7468: F, t26003: F, t4034: F) -> (F, F, F, F, F) {
    let t90444 = F::cast_from(12.0_f64) * t90442 * t24996;
    let t90447 = F::cast_from(12.0_f64) * t24995 * t34475 * t5308;
    let t90450 = F::cast_from(2.0_f64) * t1983 * t26503 * t6999;
    let t90454 = F::cast_from(2.0_f64) * t12823 * t7468;
    let t90456 = F::cast_from(4.0_f64) * t4034 * t26003;
    (t90444, t90447, t90450, t90454, t90456)
}
