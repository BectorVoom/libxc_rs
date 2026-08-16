//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 52 (v4rho2sigma2_8) CSE chunk 634/1400 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part52_v4rho2sigma2_8_chunk634<F: Float>(t1055: F, t4693: F, t1052: F, t1066: F, t1635: F, t3026: F, t3169: F, t388: F, t4553: F, t4555: F, t4557: F, t4559: F, t4658: F, t4660: F, t4665: F) -> (F, F) {
    let t4694 = t1055 * t4693;
    let t4696 = F::cast_from(2.0_f64) * t1052 * t4665 - t1052 * t4694 - t1066 * t4557 - t1066 * t4660 - t1635 * t3026 - t1635 * t3169 + t388 * t4553 + t388 * t4555 + t388 * t4559 + t388 * t4658;
    (t4694, t4696)
}
