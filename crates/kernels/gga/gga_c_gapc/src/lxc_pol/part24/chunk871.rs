//! GGA_C_GAPC lxc pol — lxc_pol part 24 (v4rho2sigma2_3) CSE chunk 871/1327 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part24_v4rho2sigma2_3_chunk871<F: Float>(t8483: F, t8487: F, t8490: F, t8494: F, t8496: F, t8498: F, t8502: F, t8506: F, t8512: F, t8515: F, t8517: F, t8522: F, t8526: F, t8529: F, t8532: F, t8536: F, t8539: F, t8541: F, t8543: F, t8547: F, t8550: F, t8553: F) -> (F, F) {
    let t10496 = F::cast_from(0.32442010650387372095e-3_f64) * t8483 - F::cast_from(0.41711156550498049836e-2_f64) * t8487 - F::cast_from(0.35429749433031915543e-4_f64) * t8490 + F::cast_from(0.59049582388386525904e-5_f64) * t8494 - F::cast_from(0.57681894936388747585e-3_f64) * t8496 + F::cast_from(0.24720812115595177536e-3_f64) * t8498 + F::cast_from(0.1545050757224698596e-4_f64) * t8502 - F::cast_from(0.91570008211517136796e-6_f64) * t8506 - F::cast_from(0.55442889295095606609e-5_f64) * t8512 + F::cast_from(0.13903718850166016612e-3_f64) * t8515 - F::cast_from(0.52638484871933131664e-3_f64) * t8517;
    let t10509 = F::cast_from(0.60736713313768998073e-4_f64) * t8522 + F::cast_from(0.40491142209179332048e-4_f64) * t8526 + F::cast_from(0.40491142209179332048e-4_f64) * t8529 - F::cast_from(0.60736713313768998073e-4_f64) * t8532 - F::cast_from(0.19683194129462175301e-5_f64) * t8536 - F::cast_from(0.60736713313768998073e-4_f64) * t8539 + F::cast_from(0.52638484871933131664e-3_f64) * t8541 + F::cast_from(0.52638484871933131664e-3_f64) * t8543 - F::cast_from(0.12147342662753799615e-3_f64) * t8547 - F::cast_from(0.12147342662753799615e-3_f64) * t8550 + F::cast_from(0.86898242813537603826e-4_f64) * t8553;
    (t10496, t10509)
}
