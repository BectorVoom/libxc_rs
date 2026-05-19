//! GGA_C_GAPC lxc pol — lxc_pol part 34 (v4rho2sigma2_13) CSE chunk 779/1427 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part34_v4rho2sigma2_13_chunk779<F: Float>(t172: F, t200: F, t6: F, t103: F, t8948: F, t4048: F, t667: F, t4043: F, t169: F, t134: F, t674: F, t1662: F, t3031: F) -> (F, F, F, F, F, F, F, F, F) {
    let pi = F::cast_from(M_PI);
    let t8950 = t6 * t200 * t172;
    let t8951 = t103 * t8950;
    let t8952 = t8948 * t8951;
    let t8954 = t4048 * t667;
    let t8955 = t8954 * t4043;
    let t8956 = t169 * t8955;
    let t8957 = pi * t6;
    let t8958 = t134 * t674;
    let t8959 = t8958 * t172;
    let t8960 = t8957 * t8959;
    let t8961 = t8956 * t8960;
    let t8963 = t1662 * t3031;
    (t8950, t8951, t8952, t8957, t8958, t8959, t8960, t8961, t8963)
}
