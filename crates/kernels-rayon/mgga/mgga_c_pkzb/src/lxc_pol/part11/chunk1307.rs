//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1307/1340 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1307(t10115: f64, t300: f64, t10044: f64, t10047: f64, t10055: f64, t10205: f64, t10209: f64, t10215: f64, t2380: f64, t27044: f64, t28033: f64, t28040: f64, t31668: f64, t3185: f64, t3188: f64, t3195: f64, t3206: f64, t3208: f64, t3223: f64, t6518: f64, t6526: f64, t8264: f64, t8319: f64, t8428: f64, t8435: f64) -> f64 {
    let t31686 = t300 * t10115;
    let t31693 = -0.41159057393346947493e-1_f64 * t8319 * t10215 + 0.27439371595564631662e-1_f64 * t10044 * t10209 - 0.13719685797782315831e-1_f64 * t10047 * t10205 - 0.1543464652250510531e-1_f64 * t2380 * t28040 * t3195 + 0.7717323261252552655e-2_f64 * t2380 * t8264 * t10055 + 0.7717323261252552655e-2_f64 * t3185 * t31668 * t3188 - 0.38586616306262763275e-2_f64 * t3206 * t31668 * t3208 - 0.77173232612525526549e-2_f64 * t8428 * t27044 * t6518 * t3223 + 0.7717323261252552655e-2_f64 * t8435 * t27044 * t6526 * t3223 + 0.38586616306262763276e-2_f64 * t2380 * t28033 * t3195 - 0.25724410870841842184e-2_f64 * t3185 * t31686 * t3188 + 0.12862205435420921092e-2_f64 * t3206 * t31686 * t3208;
    t31693
}
