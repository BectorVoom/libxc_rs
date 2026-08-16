//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 1159/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk1159(t31612: f64, t31619: f64, t31625: f64, t31627: f64, t31607: f64, t31609: f64, t31623: f64, t35860: f64, t35864: f64, t35866: f64, t35868: f64, t35872: f64, t35875: f64, t35877: f64, t35879: f64, t35882: f64, t35885: f64, t35887: f64) -> f64 {
    let t35890 = 0.17149607247227894789e-2_f64 * t31612;
    let t35891 = 0.18868855373762491241e-1_f64 * t31619;
    let t35893 = 0.25724410870841842184e-2_f64 * t31625;
    let t35894 = 0.51448821741683684368e-2_f64 * t31627;
    let t35895 = 0.1528125e-1_f64 * t35860 - 0.7862023072401038017e-3_f64 * t35864 + 0.68598428988911579156e-2_f64 * t35866 - 0.68598428988911579156e-2_f64 * t35868 + 0.94344276868812456204e-3_f64 * t35872 - t35875 + t35877 - 11.0_f64 / 192.0_f64 * t31607 - t35879 / 96.0_f64 - t35882 / 128.0_f64 - t35885 / 384.0_f64 - t35887 / 24.0_f64 - 0.19293308153131381637e-1_f64 * t31609 + t35890 + t35891 - 0.21437009059034868486e-3_f64 * t31623 + t35893 + t35894;
    t35895
}
