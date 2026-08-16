//! GGA_C_GAPLOC lxc pol — lxc_pol part 27 (v4rho2sigma2_10) CSE chunk 1135/1468 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part27_v4rho2sigma2_10_chunk1135<F: Float>(t3130: F, t6338: F, t1217: F, t29874: F, t9208: F, t4325: F, t6515: F, t6525: F, t484: F, t9090: F, t20395: F, t493: F) -> (F, F, F, F, F, F) {
    let t30182 = F::cast_from(0.23712505529730124666e-2_f64) * t6338 * t3130;
    let t30184 = F::cast_from(0.73772239425827054516e-2_f64) * t1217 * t3130;
    let t30186 = F::cast_from(0.94850022118920498664e-2_f64) * t29874 * t9208;
    let t30189 = F::cast_from(0.142275033178380748e-1_f64) * t6525 * t6515 * t4325;
    let t30199 = F::cast_from(0.63233348079280332442e-2_f64) * t484 * t9090;
    let t30200 = t493 * t20395;
    (t30182, t30184, t30186, t30189, t30199, t30200)
}
