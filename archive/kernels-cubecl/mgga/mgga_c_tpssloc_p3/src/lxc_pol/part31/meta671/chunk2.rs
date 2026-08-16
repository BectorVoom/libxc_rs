//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 31 (v4rho3sigma_7) CSE chunk 2003/2041 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk2003<F: Float>(t1409: F, t605: F, t63: F, t27961: F, t84219: F, t2032: F, t26063: F, t26070: F, t26073: F, t26076: F, t26911: F, t26945: F, t27982: F, t7035: F, t7432: F, t7435: F, t7782: F, t91907: F, t96553: F, t96556: F, t96559: F, t96562: F) -> F {
    let t102227 = t605 * t1409 * t63;
    let t102248 = t84219 * t27961;
    let t102252 = -F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t102227 * t96553 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t96556 * t2032 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t96559 * t2032 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t96562 * t2032 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t27982 * t7035 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t26070 * t7782 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t26073 * t7782 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t26076 * t7782 - F::cast_from(4.0_f64) / F::cast_from(3.0_f64) * t7435 * t26945 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t91907 * t7432 - F::cast_from(80.0_f64) / F::cast_from(3.0_f64) * t102248 - F::cast_from(10.0_f64) / F::cast_from(3.0_f64) * t26911 * t26063;
    t102252
}
