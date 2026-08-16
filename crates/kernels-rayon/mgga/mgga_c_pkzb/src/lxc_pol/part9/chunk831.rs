//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 831/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk831(t2033: f64, t2104: f64, t2108: f64, t2899: f64, t2922: f64, t5925: f64, t5929: f64, t5933: f64, t5936: f64, t5941: f64, t5945: f64, t5948: f64, t5954: f64, t5958: f64, t5962: f64, t5967: f64, t5971: f64, t5976: f64, t5980: f64, t5984: f64, t763: f64) -> f64 {
    let t5987 = -0.68598428988911579154e-2_f64 * t5925 * t2033 + 0.85748036236139473944e-3_f64 * t5929 + 0.21437009059034868486e-3_f64 * t5933 * t5936 - 0.14291339372689912324e-3_f64 * t5941 + 0.21722835846488666732e-1_f64 * t5945 * t763 - 0.45732285992607719436e-2_f64 * t5948 + 0.12862205435420921092e-2_f64 * t5954 * t5958 - 0.64311027177104605458e-3_f64 * t2922 * t5962 - 0.25724410870841842183e-2_f64 * t2899 * t5967 + 0.12862205435420921092e-2_f64 * t2922 * t5971 - 0.17149607247227894789e-2_f64 * t5976 - 0.12862205435420921092e-2_f64 * t2104 * t5980 + 0.13719685797782315831e-1_f64 * t5984 * t2108;
    t5987
}
