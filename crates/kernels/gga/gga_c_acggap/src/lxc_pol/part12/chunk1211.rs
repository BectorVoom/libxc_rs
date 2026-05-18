//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1211/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1211<F: Float>(t35997: F, t35965: F, t35971: F, t35991: F, t35995: F, t35999: F, t37806: F, t37807: F, t37808: F, t37810: F, t37811: F, t37813: F, t37814: F, t37815: F, t37816: F, t37817: F, t37818: F, t37819: F) -> F {
    let t37822 = F::new(0.18868855373762491241e-1) * t35997;
    let t37824 = t37806 + t37807 + t37808 + F::new(0.17149607247227894789e-2) * t35965 - t37810 + t37811 - F::new(0.17149607247227894789e-2) * t35971 - t37813 - t37814 + t37815 - t37816 + t37817 + t37818 + t37819 + F::new(0.41930789719472202757e-2) * t35991 - F::new(0.62896184579208304135e-2) * t35995 - t37822 + F::new(0.68598428988911579156e-2) * t35999;
    t37824
}
