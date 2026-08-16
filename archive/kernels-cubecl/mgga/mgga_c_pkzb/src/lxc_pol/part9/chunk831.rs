//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 831/1336 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk831<F: Float>(t2033: F, t2104: F, t2108: F, t2899: F, t2922: F, t5925: F, t5929: F, t5933: F, t5936: F, t5941: F, t5945: F, t5948: F, t5954: F, t5958: F, t5962: F, t5967: F, t5971: F, t5976: F, t5980: F, t5984: F, t763: F) -> F {
    let t5987 = -F::cast_from(0.68598428988911579154e-2_f64) * t5925 * t2033 + F::cast_from(0.85748036236139473944e-3_f64) * t5929 + F::cast_from(0.21437009059034868486e-3_f64) * t5933 * t5936 - F::cast_from(0.14291339372689912324e-3_f64) * t5941 + F::cast_from(0.21722835846488666732e-1_f64) * t5945 * t763 - F::cast_from(0.45732285992607719436e-2_f64) * t5948 + F::cast_from(0.12862205435420921092e-2_f64) * t5954 * t5958 - F::cast_from(0.64311027177104605458e-3_f64) * t2922 * t5962 - F::cast_from(0.25724410870841842183e-2_f64) * t2899 * t5967 + F::cast_from(0.12862205435420921092e-2_f64) * t2922 * t5971 - F::cast_from(0.17149607247227894789e-2_f64) * t5976 - F::cast_from(0.12862205435420921092e-2_f64) * t2104 * t5980 + F::cast_from(0.13719685797782315831e-1_f64) * t5984 * t2108;
    t5987
}
