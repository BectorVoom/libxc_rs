//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1060/1092 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1060<F: Float>(t35959: F, t35961: F, t35963: F, t35967: F, t35969: F, t35973: F, t35975: F, t35977: F, t35979: F, t35981: F, t35985: F, t35987: F, t35997: F, t35965: F, t35971: F, t35991: F, t35995: F, t35999: F) -> (F,) {
    let t37806 = 0.17149607247227894789e-2 * t35959;
    let t37807 = 0.34299214494455789578e-2 * t35961;
    let t37808 = 0.34299214494455789578e-2 * t35963;
    let t37810 = 0.13719685797782315831e-1 * t35967;
    let t37811 = 0.16006300097412701803e-1 * t35969;
    let t37813 = 0.16006300097412701803e-1 * t35973;
    let t37814 = 0.34299214494455789578e-2 * t35975;
    let t37815 = 0.34299214494455789578e-2 * t35977;
    let t37816 = 0.17149607247227894789e-2 * t35979;
    let t37817 = 0.17149607247227894789e-2 * t35981;
    let t37818 = 0.14291339372689912324e-2 * t35985;
    let t37819 = 0.68598428988911579156e-2 * t35987;
    let t37822 = 0.18868855373762491241e-1 * t35997;
    let t37824 = t37806 + t37807 + t37808 + 0.17149607247227894789e-2 * t35965 - t37810 + t37811 - 0.17149607247227894789e-2 * t35971 - t37813 - t37814 + t37815 - t37816 + t37817 + t37818 + t37819 + 0.41930789719472202757e-2 * t35991 - 0.62896184579208304135e-2 * t35995 - t37822 + 0.68598428988911579156e-2 * t35999;
    (t37824,)
}
