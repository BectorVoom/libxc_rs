//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 38 (v4rho3tau_2) CSE chunk 1320/1335 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part38_v4rho3tau_2_chunk1320<F: Float>(t111: F, t8153: F, t112: F, t29978: F, t30017: F, t576: F, t1851: F, t8171: F, t110140: F, t8223: F, t29895: F, t30152: F) -> (F, F, F, F, F, F) {
    let t110240 = t8153 * t111;
    let t110253 = t29978 * t112;
    let t110268 = t576 * t30017;
    let t110489 = F::cast_from(2.0_f64) * t1851 * t8171;
    let t110503 = t110140 * t8223;
    let t110506 = F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t29895 * t30152;
    (t110240, t110253, t110268, t110489, t110503, t110506)
}
