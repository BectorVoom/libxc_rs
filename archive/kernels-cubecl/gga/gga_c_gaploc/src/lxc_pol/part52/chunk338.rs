//! GGA_C_GAPLOC lxc pol — lxc_pol part 52 (v4rhosigma3_17) CSE chunk 338/1013 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part52_v4rhosigma3_17_chunk338<F: Float>(t2717: F, t313: F, t2013: F, t970: F, t2465: F, t325: F, t2464: F, t825: F, t2028: F, t2033: F, t2049: F, t2087: F, t2194: F, t2197: F, t2664: F, t2669: F, t2673: F, t2676: F, t2681: F, t2687: F, t2689: F, t2692: F, t2699: F, t2705: F, t2711: F, t2714: F, t317: F, t784: F, t797: F, t813: F, t833: F, t955: F, t962: F, t966: F, t974: F) -> (F, F, F, F) {
    let t2718 = t313 * t2717;
    let t2721 = t2013 * t970;
    let t2723 = t2465 * t325;
    let t2724 = t2464 * t2723;
    let t2725 = t825 * t2724;
    let t2727 = F::cast_from(0.11502877786176224903e2_f64) * t833 * t2664 - F::cast_from(0.69017266717057349418e1_f64) * t2087 * t2669 - F::cast_from(0.39722766613167140743e-1_f64) * t2673 * t2028 + F::cast_from(0.39722766613167140743e-1_f64) * t2033 * t2676 - F::cast_from(0.19171462976960374838e0_f64) * t2681 + F::cast_from(0.19171462976960374838e0_f64) * t2687 - F::cast_from(0.23833659967900284446e0_f64) * t797 * t2689 - F::cast_from(0.30674340763136599741e1_f64) * t813 * t2692 + F::cast_from(0.23833659967900284446e0_f64) * t955 * t784 - F::cast_from(0.35750489951850426669e0_f64) * t2049 * t962 - F::cast_from(0.35750489951850426669e0_f64) * t797 * t2699 - F::cast_from(0.23005755572352449806e1_f64) * t2194 * t966 - F::cast_from(0.23005755572352449806e1_f64) * t813 * t2705 + F::cast_from(0.23005755572352449806e1_f64) * t2197 * t974 + F::cast_from(0.23005755572352449806e1_f64) * t833 * t2711 + F::cast_from(0.35750489951850426669e0_f64) * t2714 * t317 + F::cast_from(0.35750489951850426669e0_f64) * t2718 * t317 - F::cast_from(0.95857314884801874192e-1_f64) * t2721 + F::cast_from(0.21301625529955972043e-1_f64) * t2725;
    (t2718, t2721, t2725, t2727)
}
