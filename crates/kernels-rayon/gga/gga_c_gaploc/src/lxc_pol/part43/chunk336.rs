//! GGA_C_GAPLOC lxc pol — lxc_pol part 43 (v4rhosigma3_8) CSE chunk 336/1072 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part43_v4rhosigma3_8_chunk336(t2724: f64, t825: f64, t2028: f64, t2033: f64, t2049: f64, t2087: f64, t2194: f64, t2197: f64, t2664: f64, t2669: f64, t2673: f64, t2676: f64, t2681: f64, t2687: f64, t2689: f64, t2692: f64, t2699: f64, t2705: f64, t2711: f64, t2714: f64, t2718: f64, t2721: f64, t317: f64, t784: f64, t797: f64, t813: f64, t833: f64, t955: f64, t962: f64, t966: f64, t974: f64) -> (f64, f64) {
    let t2725 = t825 * t2724;
    let t2727 = 0.11502877786176224903e2_f64 * t833 * t2664 - 0.69017266717057349418e1_f64 * t2087 * t2669 - 0.39722766613167140743e-1_f64 * t2673 * t2028 + 0.39722766613167140743e-1_f64 * t2033 * t2676 - 0.19171462976960374838e0_f64 * t2681 + 0.19171462976960374838e0_f64 * t2687 - 0.23833659967900284446e0_f64 * t797 * t2689 - 0.30674340763136599741e1_f64 * t813 * t2692 + 0.23833659967900284446e0_f64 * t955 * t784 - 0.35750489951850426669e0_f64 * t2049 * t962 - 0.35750489951850426669e0_f64 * t797 * t2699 - 0.23005755572352449806e1_f64 * t2194 * t966 - 0.23005755572352449806e1_f64 * t813 * t2705 + 0.23005755572352449806e1_f64 * t2197 * t974 + 0.23005755572352449806e1_f64 * t833 * t2711 + 0.35750489951850426669e0_f64 * t2714 * t317 + 0.35750489951850426669e0_f64 * t2718 * t317 - 0.95857314884801874192e-1_f64 * t2721 + 0.21301625529955972043e-1_f64 * t2725;
    (t2725, t2727)
}
