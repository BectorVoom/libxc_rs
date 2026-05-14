//! GGA_C_GAPLOC lxc pol — lxc_pol part 30 (v4rho2sigma2_13) CSE chunk 1249/1268 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part30_v4rho2sigma2_13_chunk1249<F: Float>(t34371: F, t6710: F, t6711: F, t34321: F, t6717: F, t6914: F, t10532: F, t10533: F, t204: F, t587: F, t2487: F, t31190: F, t31213: F, t31215: F, t31217: F, t34950: F, t34953: F, t34954: F, t34957: F, t34959: F, t34962: F, t34964: F) -> (F,) {
    let t34967 = 0.23005755572352449806e2 * t6710 * t6711 * t34371;
    let t34970 = 0.12423108009070322895e3 * t6914 * t6717 * t34321;
    let t34973 = 0.55213813373645879534e2 * t10532 * t10533 * t34321;
    let t34976 = 0.18404604457881959845e2 * t587 * t204 * t34321;
    let t34979 = 0.87421871174939309262e2 * t2487 * t6711 * t34321;
    let t34980 = t34950 + t34953 - t31190 - t34954 - t31213 - t31215 + t31217 + t34957 - t34959 + t34962 - t34964 - t34967 - t34970 + t34973 - t34976 + t34979;
    (t34980,)
}
