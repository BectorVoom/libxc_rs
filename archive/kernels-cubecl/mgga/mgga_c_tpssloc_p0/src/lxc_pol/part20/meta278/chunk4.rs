//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 1462/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk1462<F: Float>(t10544: F, t10530: F, t10538: F, t10556: F, t10558: F, t10560: F, t10562: F, t10566: F, t10569: F, t10572: F, t10575: F, t894: F) -> (F, F, F) {
    let t10577 = F::cast_from(28.0_f64) / F::cast_from(27.0_f64) * t10544;
    let t10588 = -t10577 - F::cast_from(4.0_f64) / F::cast_from(9.0_f64) * t10556 + F::cast_from(2.0_f64) / F::cast_from(9.0_f64) * t10558 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t10560 + t10562 / F::cast_from(3.0_f64) - F::cast_from(10.0_f64) / F::cast_from(27.0_f64) * t10566 + F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t10569 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t10530 - F::cast_from(2.0_f64) * t10572 + F::cast_from(2.0_f64) * t10538 - t10575 / F::cast_from(3.0_f64);
    let t10589 = t894 * t10588;
    (t10577, t10588, t10589)
}
