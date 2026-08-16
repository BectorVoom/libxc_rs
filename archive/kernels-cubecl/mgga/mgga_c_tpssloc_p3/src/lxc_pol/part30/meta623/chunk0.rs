//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2023/2341 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2023<F: Float>(t86942: F, t23168: F, t25338: F, t23012: F, t7485: F, t25046: F, t6579: F, t1484: F, t2717: F, t225: F, t25051: F, t7489: F) -> (F, F, F, F, F, F, F) {
    let t86943 = F::cast_from(0.38381794893125283518e-1_f64) * t86942;
    let t86950 = t23168 * t25338;
    let t86951 = F::cast_from(0.76763589786250567036e-1_f64) * t86950;
    let t86955 = t23012 * t7485;
    let t86967 = t6579 * t25046;
    let t86968 = F::cast_from(0.76763589786250567036e-1_f64) * t86967;
    let t86969 = t2717 * t1484;
    let t86988 = t25051 * t225;
    let t86991 = t23012 * t7489;
    (t86943, t86951, t86955, t86968, t86969, t86988, t86991)
}
