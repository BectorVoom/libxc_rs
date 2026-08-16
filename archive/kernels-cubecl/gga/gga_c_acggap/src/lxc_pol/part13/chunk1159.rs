//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1159/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1159<F: Float>(t31612: F, t31619: F, t31625: F, t31627: F, t31607: F, t31609: F, t31623: F, t35860: F, t35864: F, t35866: F, t35868: F, t35872: F, t35875: F, t35877: F, t35879: F, t35882: F, t35885: F, t35887: F) -> F {
    let t35890 = F::cast_from(0.17149607247227894789e-2_f64) * t31612;
    let t35891 = F::cast_from(0.18868855373762491241e-1_f64) * t31619;
    let t35893 = F::cast_from(0.25724410870841842184e-2_f64) * t31625;
    let t35894 = F::cast_from(0.51448821741683684368e-2_f64) * t31627;
    let t35895 = F::cast_from(0.1528125e-1_f64) * t35860 - F::cast_from(0.7862023072401038017e-3_f64) * t35864 + F::cast_from(0.68598428988911579156e-2_f64) * t35866 - F::cast_from(0.68598428988911579156e-2_f64) * t35868 + F::cast_from(0.94344276868812456204e-3_f64) * t35872 - t35875 + t35877 - F::cast_from(11.0_f64) / F::cast_from(192.0_f64) * t31607 - t35879 / F::cast_from(96.0_f64) - t35882 / F::cast_from(128.0_f64) - t35885 / F::cast_from(384.0_f64) - t35887 / F::cast_from(24.0_f64) - F::cast_from(0.19293308153131381637e-1_f64) * t31609 + t35890 + t35891 - F::cast_from(0.21437009059034868486e-3_f64) * t31623 + t35893 + t35894;
    t35895
}
