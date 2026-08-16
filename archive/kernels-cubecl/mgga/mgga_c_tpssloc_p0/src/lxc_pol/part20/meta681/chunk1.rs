//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2568/2712 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2568<F: Float>(t11598: F, t11919: F, t11935: F, t1238: F, t1251: F, t1252: F, t14972: F, t15786: F, t15794: F, t15797: F, t15803: F, t15820: F, t1751: F, t1761: F, t3487: F, t3598: F, t3600: F, t3631: F, t44412: F, t4945: F, t498: F, t51925: F, t51928: F, t51937: F) -> F {
    let t51946 = F::cast_from(6.0_f64) * t1238 * t1251 * t15786 * t3598 + t11598 * t1751 * t498 - t11919 * t4945 + F::cast_from(6.0_f64) * t11935 * t4945 - F::cast_from(6.0_f64) * t1252 * t51925 - F::cast_from(3.0_f64) * t1252 * t51928 - F::cast_from(3.0_f64) * t1252 * t51937 + F::cast_from(6.0_f64) * t14972 * t3600 - F::cast_from(18.0_f64) * t15794 * t3487 - F::cast_from(3.0_f64) * t15797 * t3631 + F::cast_from(6.0_f64) * t15803 * t3487 + F::cast_from(6.0_f64) * t15820 * t3600 - t1761 * t44412;
    t51946
}
