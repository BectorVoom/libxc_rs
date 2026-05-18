//! GGA_C_ACGGAP lxc pol — lxc_pol part 13 (v4rho3sigma_5) CSE chunk 1159/1213 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part13_v4rho3sigma_5_chunk1159<F: Float>(t31612: F, t31619: F, t31625: F, t31627: F, t31607: F, t31609: F, t31623: F, t35860: F, t35864: F, t35866: F, t35868: F, t35872: F, t35875: F, t35877: F, t35879: F, t35882: F, t35885: F, t35887: F) -> F {
    let t35890 = F::new(0.17149607247227894789e-2) * t31612;
    let t35891 = F::new(0.18868855373762491241e-1) * t31619;
    let t35893 = F::new(0.25724410870841842184e-2) * t31625;
    let t35894 = F::new(0.51448821741683684368e-2) * t31627;
    let t35895 = F::new(0.1528125e-1) * t35860 - F::new(0.7862023072401038017e-3) * t35864 + F::new(0.68598428988911579156e-2) * t35866 - F::new(0.68598428988911579156e-2) * t35868 + F::new(0.94344276868812456204e-3) * t35872 - t35875 + t35877 - F::new(11.0) / F::new(192.0) * t31607 - t35879 / F::new(96.0) - t35882 / F::new(128.0) - t35885 / F::new(384.0) - t35887 / F::new(24.0) - F::new(0.19293308153131381637e-1) * t31609 + t35890 + t35891 - F::new(0.21437009059034868486e-3) * t31623 + t35893 + t35894;
    t35895
}
