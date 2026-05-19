//! MGGA_C_KCIS lxc pol — lxc_pol part 26 (v4rho3sigma_8) CSE chunk 817/1397 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part26_v4rho3sigma_8_chunk817<F: Float>(t1341: F, t3944: F, t11388: F, t473: F, t11536: F, t3918: F, t1559: F, t4330: F, t4355: F, t11407: F, t110: F, t1369: F) -> (F, F, F, F, F, F, F, F) {
    let t12741 = t1341 * t3944;
    let t12744 = t473 * t11388;
    let t12751 = t473 * t11536;
    let t12755 = t1341 * t3918;
    let t12761 = t1559 * t4330;
    let t12772 = t1559 * t4355;
    let t12791 = F::cast_from(0.53272592592592592592e-1_f64) * t11407;
    let t12825 = t110 * t1369;
    (t12741, t12744, t12751, t12755, t12761, t12772, t12791, t12825)
}
