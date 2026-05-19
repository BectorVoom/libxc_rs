//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 665/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk665<F: Float>(t10536: F, t1869: F, t4736: F, t4744: F, t1663: F, t4742: F, t45: F, t4753: F, t4781: F, t4787: F, t6880: F, t1683: F, t4762: F) -> (F, F, F, F, F) {
    let t10537 = t1869 * t10536;
    let t10539 = t4736 * t4744;
    let t10540 = t10539 * t1663;
    let t10542 = F::cast_from(0.48245472966453314466e2_f64) * t4742 * t10540;
    let t10543 = t45 * t4753;
    let t10549 = t4787 * t4781 * t6880;
    let t10552 = t4762 * t1683;
    (t10537, t10542, t10543, t10549, t10552)
}
