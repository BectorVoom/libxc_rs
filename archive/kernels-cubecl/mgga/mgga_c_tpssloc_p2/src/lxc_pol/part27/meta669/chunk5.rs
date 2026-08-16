//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2368/2372 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2368<F: Float>(t90030: F, t90422: F, t91574: F, t91617: F, t91663: F, t91709: F, t91750: F, t91789: F, t26135: F, t3941: F, t671: F, t2363: F, t7467: F) -> (F, F, F) {
    let t91792 = t90030 + t90422 + t91574 + t91617 + t91663 + t91709 + t91750 + t91789;
    let t91799 = F::cast_from(54.0_f64) * t3941 * t26135 * t671;
    let t91802 = F::cast_from(27.0_f64) * t3941 * t7467 * t2363;
    (t91792, t91799, t91802)
}
