//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1437/1497 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1437<F: Float>(t43748: F, t43750: F, t43780: F, t43782: F, t43784: F, t43786: F, t43788: F, t43794: F, t43798: F, t43802: F, t43806: F, t43819: F) -> (F, F) {
    let t44342 = -F::cast_from(0.31659259259259259258e-1_f64) * t43748 - F::cast_from(0.26382716049382716049e-1_f64) * t43750 + F::cast_from(0.47488888888888888888e-1_f64) * t43780 + F::cast_from(0.94977777777777777776e-1_f64) * t43782 + F::cast_from(0.94977777777777777776e-1_f64) * t43784 - F::cast_from(0.14246666666666666667e0_f64) * t43786 - F::cast_from(0.23744444444444444444e-1_f64) * t43788 + F::cast_from(0.23744444444444444444e0_f64) * t43794 - F::cast_from(0.42739999999999999999e0_f64) * t43798 + F::cast_from(0.4274e0_f64) * t43802 + F::cast_from(0.17808333333333333333e-1_f64) * t43806;
    let t44348 = F::cast_from(0.18467901234567901234e0_f64) * t43819;
    (t44342, t44348)
}
