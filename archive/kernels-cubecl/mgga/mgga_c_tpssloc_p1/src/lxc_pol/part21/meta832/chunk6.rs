//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2938/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2938<F: Float>(t135: F, t17843: F, t973: F, t13831: F, t17804: F, t2986: F, t2988: F, t340: F, t343: F, t42811: F, t42817: F, t42873: F, t42877: F, t42893: F, t42895: F, t4531: F, t47887: F, t47938: F, t61103: F, t61124: F, t61138: F, t61150: F, t61163: F, t974: F) -> F {
    let t61172 = t973 * t135 * t17843;
    let t61181 = F::cast_from(0.22222222222222222222e-2_f64) * t2986 * t2988 * t61103 - F::cast_from(0.33333333333333333333e-2_f64) * t2986 * t4531 * t47887 + F::cast_from(0.74074074074074074072e-3_f64) * t47938 - F::cast_from(0.16460905349794238683e-2_f64) * t42811 - t42817 - F::cast_from(0.83333333333333333332e-3_f64) * t973 * t974 * t340 * (t61124 + t61138 + t61150 + t61163) * t343 - F::cast_from(0.55555555555555555554e-3_f64) * t61172 - F::cast_from(0.6172839506172839506e-4_f64) * t42873 - F::cast_from(0.82304526748971193413e-4_f64) * t42877 + F::cast_from(0.20576131687242798354e-3_f64) * t42893 - F::cast_from(0.18106995884773662551e-2_f64) * t42895 - F::cast_from(0.55555555555555555554e-3_f64) * t2986 * t17804 * t13831;
    t61181
}
