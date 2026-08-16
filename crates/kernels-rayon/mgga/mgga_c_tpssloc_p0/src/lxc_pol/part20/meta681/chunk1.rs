//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2568/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2568(t11598: f64, t11919: f64, t11935: f64, t1238: f64, t1251: f64, t1252: f64, t14972: f64, t15786: f64, t15794: f64, t15797: f64, t15803: f64, t15820: f64, t1751: f64, t1761: f64, t3487: f64, t3598: f64, t3600: f64, t3631: f64, t44412: f64, t4945: f64, t498: f64, t51925: f64, t51928: f64, t51937: f64) -> f64 {
    let t51946 = 6.0_f64 * t1238 * t1251 * t15786 * t3598 + t11598 * t1751 * t498 - t11919 * t4945 + 6.0_f64 * t11935 * t4945 - 6.0_f64 * t1252 * t51925 - 3.0_f64 * t1252 * t51928 - 3.0_f64 * t1252 * t51937 + 6.0_f64 * t14972 * t3600 - 18.0_f64 * t15794 * t3487 - 3.0_f64 * t15797 * t3631 + 6.0_f64 * t15803 * t3487 + 6.0_f64 * t15820 * t3600 - t1761 * t44412;
    t51946
}
