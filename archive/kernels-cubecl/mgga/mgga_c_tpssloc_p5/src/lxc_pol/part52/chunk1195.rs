//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 1195/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk1195<F: Float>(t32670: F, t652: F, t4028: F, t8327: F, t7458: F, t1774: F, t8326: F, t1874: F, t24999: F, t7685: F, t8490: F, t1842: F, t8485: F) -> (F, F, F, F, F, F, F, F) {
    let t32671 = t652 * t32670;
    let t32673 = t4028 * t8327;
    let t32674 = F::cast_from(2.0_f64) * t32673;
    let t32675 = t7458 * t8327;
    let t32676 = F::cast_from(2.0_f64) * t32675;
    let t32677 = t1774 * t8326;
    let t32678 = t652 * t32677;
    let t32679 = F::cast_from(2.0_f64) * t32678;
    let t32680 = t24999 * t1874;
    let t32684 = t7685 * t8490;
    let t32685 = t8485 * t1842;
    (t32671, t32674, t32676, t32677, t32679, t32680, t32684, t32685)
}
