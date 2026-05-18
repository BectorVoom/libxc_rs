//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1307/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1307<F: Float>(t10115: F, t300: F, t10044: F, t10047: F, t10055: F, t10205: F, t10209: F, t10215: F, t2380: F, t27044: F, t28033: F, t28040: F, t31668: F, t3185: F, t3188: F, t3195: F, t3206: F, t3208: F, t3223: F, t6518: F, t6526: F, t8264: F, t8319: F, t8428: F, t8435: F) -> F {
    let t31686 = t300 * t10115;
    let t31693 = -F::new(0.41159057393346947493e-1) * t8319 * t10215 + F::new(0.27439371595564631662e-1) * t10044 * t10209 - F::new(0.13719685797782315831e-1) * t10047 * t10205 - F::new(0.1543464652250510531e-1) * t2380 * t28040 * t3195 + F::new(0.7717323261252552655e-2) * t2380 * t8264 * t10055 + F::new(0.7717323261252552655e-2) * t3185 * t31668 * t3188 - F::new(0.38586616306262763275e-2) * t3206 * t31668 * t3208 - F::new(0.77173232612525526549e-2) * t8428 * t27044 * t6518 * t3223 + F::new(0.7717323261252552655e-2) * t8435 * t27044 * t6526 * t3223 + F::new(0.38586616306262763276e-2) * t2380 * t28033 * t3195 - F::new(0.25724410870841842184e-2) * t3185 * t31686 * t3188 + F::new(0.12862205435420921092e-2) * t3206 * t31686 * t3208;
    t31693
}
