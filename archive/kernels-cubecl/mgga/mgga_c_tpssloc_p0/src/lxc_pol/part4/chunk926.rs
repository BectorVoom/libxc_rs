//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 4 (v3rho3_2) CSE chunk 926/1228 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part4_v3rho3_2_chunk926<F: Float>(t1472: F, t2517: F, t4303: F, t870: F, t2430: F, t4205: F, t1409: F, t750: F, t607: F, t4194: F, t3966: F, t751: F) -> (F, F, F, F, F) {
    let t12861 = t1472 * t2517;
    let t12895 = t4303 * t870;
    let t12922 = F::cast_from(8.0_f64) * t4205 * t2430;
    let t12923 = t750 * t1409;
    let t12924 = t12923 * t607;
    let t12926 = F::cast_from(24.0_f64) * t4194 * t12924;
    let t12932 = t751 * t3966;
    (t12861, t12895, t12922, t12926, t12932)
}
