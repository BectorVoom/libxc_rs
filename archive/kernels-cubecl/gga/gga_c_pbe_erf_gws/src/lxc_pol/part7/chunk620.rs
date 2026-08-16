//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 620/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk620<F: Float>(t4821: F, t1423: F, t409: F, t1333: F, t461: F, t1438: F, t428: F, t4688: F, t4711: F, t4714: F, t4718: F, t4811: F, t4815: F, t4818: F, t4820: F) -> (F, F, F, F, F, F, F, F) {
    let t4822 = F::cast_from(24.0_f64) * t4821;
    let t4823 = t409 * t1423;
    let t4824 = F::cast_from(12.0_f64) * t4823;
    let t4825 = t1333 * t461;
    let t4826 = F::cast_from(60.0_f64) * t4825;
    let t4827 = t1438 * t428;
    let t4828 = F::cast_from(96.0_f64) * t4827;
    let t4829 = t4811 - t4815 + t4688 + t4711 - t4714 - t4718 - t4818 + t4820 - t4822 + t4824 + t4826 - t4828;
    (t4822, t4823, t4824, t4825, t4826, t4827, t4828, t4829)
}
