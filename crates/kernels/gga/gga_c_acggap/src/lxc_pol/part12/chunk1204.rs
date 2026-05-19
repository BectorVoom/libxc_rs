//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1204/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1204<F: Float>(t35733: F, t35736: F, t35738: F, t35740: F, t35744: F, t35747: F, t35755: F, t31544: F, t31565: F, t31570: F, t31585: F, t31593: F, t35731: F, t35742: F, t35751: F, t35753: F, t35759: F) -> F {
    let t37694 = F::cast_from(0.17149607247227894789e-2_f64) * t35733;
    let t37696 = F::cast_from(0.68598428988911579156e-2_f64) * t35736;
    let t37697 = F::cast_from(0.68598428988911579156e-2_f64) * t35738;
    let t37698 = F::cast_from(0.16006300097412701803e-1_f64) * t35740;
    let t37700 = F::cast_from(0.25724410870841842184e-2_f64) * t35744;
    let t37701 = F::cast_from(0.85748036236139473944e-3_f64) * t35747;
    let t37704 = F::cast_from(0.34299214494455789578e-1_f64) * t35755;
    let t37710 = F::cast_from(0.68598428988911579156e-2_f64) * t35731 - t37694 + F::cast_from(0.13208198761633743869e0_f64) * t31544 - t37696 + t37697 + t37698 - F::cast_from(0.68598428988911579156e-2_f64) * t35742 - t37700 - t37701 - F::cast_from(0.42874018118069736972e-2_f64) * t35751 - F::cast_from(0.13719685797782315831e-1_f64) * t35753 + t37704 - F::cast_from(0.15724046144802076034e-2_f64) * t35759 + F::cast_from(0.62896184579208304138e-3_f64) * t31565 + F::cast_from(0.12579236915841660828e-2_f64) * t31570 + F::cast_from(0.21437009059034868486e-3_f64) * t31585 - F::cast_from(0.85748036236139473944e-3_f64) * t31593;
    t37710
}
