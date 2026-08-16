//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2213/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2213<F: Float>(t51: F, t9300: F, t12606: F, t12680: F, t12698: F, t1409: F, t1420: F, t2244: F, t2250: F, t2267: F, t2274: F, t39: F, t39159: F, t39168: F, t3966: F, t3981: F, t3990: F, t45970: F, t45971: F, t607: F, t9258: F, t9287: F, t9288: F, t9305: F) -> F {
    let t45974 = t51 * t9300;
    let t45977 = -F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t39 * t9287 * t3966 * t2244 + F::cast_from(5.0_f64) / F::cast_from(162.0_f64) * t39 * t39159 * t1409 * t9288 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t39 * t2267 * t12606 * t607 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t39 * t12680 * t2250 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t39 * t3981 * t9258 - F::cast_from(20.0_f64) / F::cast_from(9.0_f64) * t1420 * t9305 + F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t51 * t9300 * t3966 * t2244 + F::cast_from(5.0_f64) / F::cast_from(162.0_f64) * t51 * t39168 * t1409 * t9288 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t51 * t2274 * t12606 * t607 + F::cast_from(5.0_f64) / F::cast_from(6.0_f64) * t51 * t12698 * t2250 + F::cast_from(5.0_f64) / F::cast_from(18.0_f64) * t51 * t3990 * t9258 - F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t45970 * t45971 + F::cast_from(5.0_f64) / F::cast_from(36.0_f64) * t45974 * t45971;
    t45977
}
