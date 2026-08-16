//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1204/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1204(t35733: f64, t35736: f64, t35738: f64, t35740: f64, t35744: f64, t35747: f64, t35755: f64, t31544: f64, t31565: f64, t31570: f64, t31585: f64, t31593: f64, t35731: f64, t35742: f64, t35751: f64, t35753: f64, t35759: f64) -> f64 {
    let t37694 = 0.17149607247227894789e-2_f64 * t35733;
    let t37696 = 0.68598428988911579156e-2_f64 * t35736;
    let t37697 = 0.68598428988911579156e-2_f64 * t35738;
    let t37698 = 0.16006300097412701803e-1_f64 * t35740;
    let t37700 = 0.25724410870841842184e-2_f64 * t35744;
    let t37701 = 0.85748036236139473944e-3_f64 * t35747;
    let t37704 = 0.34299214494455789578e-1_f64 * t35755;
    let t37710 = 0.68598428988911579156e-2_f64 * t35731 - t37694 + 0.13208198761633743869e0_f64 * t31544 - t37696 + t37697 + t37698 - 0.68598428988911579156e-2_f64 * t35742 - t37700 - t37701 - 0.42874018118069736972e-2_f64 * t35751 - 0.13719685797782315831e-1_f64 * t35753 + t37704 - 0.15724046144802076034e-2_f64 * t35759 + 0.62896184579208304138e-3_f64 * t31565 + 0.12579236915841660828e-2_f64 * t31570 + 0.21437009059034868486e-3_f64 * t31585 - 0.85748036236139473944e-3_f64 * t31593;
    t37710
}
