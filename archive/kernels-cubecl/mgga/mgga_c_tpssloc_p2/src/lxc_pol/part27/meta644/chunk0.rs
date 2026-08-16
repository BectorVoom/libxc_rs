//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2198/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2198<F: Float>(t25796: F, t4547: F, t25425: F, t82431: F, t25816: F, t3173: F, t883: F, t25443: F, t1049: F, t7577: F, t7557: F, t82573: F) -> (F, F, F, F, F, F, F) {
    let t88058 = t4547 * t25796;
    let t88069 = F::cast_from(0.36554090374405031922e-2_f64) * t82431 * t25425;
    let t88075 = F::cast_from(0.18277045187202515961e-2_f64) * t82431 * t25816;
    let t88076 = t3173 * t883;
    let t88083 = F::cast_from(0.18277045187202515961e-2_f64) * t82431 * t25443;
    let t88089 = t7577 * t1049;
    let t88096 = F::cast_from(0.14621636149762012769e-1_f64) * t82573 * t7557;
    (t88058, t88069, t88075, t88076, t88083, t88089, t88096)
}
