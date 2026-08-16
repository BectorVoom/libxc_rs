//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 51 (v4rho2sigma2_7) CSE chunk 1113/1475 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part51_v4rho2sigma2_7_chunk1113<F: Float>(t2032: F, t23975: F, t26055: F, t26063: F, t26067: F, t26070: F, t26073: F, t26076: F, t26090: F, t26911: F, t26920: F, t26936: F, t6492: F, t6495: F, t7026: F, t7035: F, t7432: F, t7435: F, t7782: F) -> F {
    let t26938 = -F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t26911 * t6492 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t26055 * t2032 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t23975 * t7432 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7026 * t26063 + F::cast_from(40.0_f64) / F::cast_from(9.0_f64) * t26920 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7026 * t26067 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t26070 * t2032 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t26073 * t2032 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t26076 * t2032 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t7435 * t7035 - F::cast_from(5.0_f64) / F::cast_from(3.0_f64) * t7026 * t26090 - F::cast_from(2.0_f64) / F::cast_from(3.0_f64) * t6495 * t7782 + F::cast_from(16.0_f64) / F::cast_from(9.0_f64) * t26936;
    t26938
}
