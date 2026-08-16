//! GGA_C_ACGGAP lxc pol — lxc_pol part 12 (v4rho3sigma_4) CSE chunk 1185/1250 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part12_v4rho3sigma_4_chunk1185<F: Float>(t35022: F, t30862: F, t30866: F, t30868: F, t30872: F, t30874: F, t30876: F, t30878: F, t30880: F, t30884: F, t30891: F, t30893: F, t30901: F, t32619: F, t32621: F, t32622: F, t32627: F, t32628: F) -> F {
    let t37345 = F::cast_from(0.57165357490759649296e-3_f64) * t35022;
    let t37358 = t37345 - F::cast_from(0.25724410870841842184e-1_f64) * t30862 - F::cast_from(0.34299214494455789578e-2_f64) * t30866 + F::cast_from(0.90702367218671976883e-1_f64) * t30868 - F::cast_from(0.90702367218671976883e-1_f64) * t30872 + F::cast_from(0.64025200389650807212e-1_f64) * t30874 + F::cast_from(0.16006300097412701803e-1_f64) * t30876 - F::cast_from(0.32012600194825403606e-1_f64) * t30878 + F::cast_from(0.18007087609589289529e-1_f64) * t30880 + t32619 - F::cast_from(0.80031500487063509015e-1_f64) * t30884 - t32621 - t32622 + F::cast_from(0.21437009059034868486e-3_f64) * t30891 + F::cast_from(0.57165357490759649296e-3_f64) * t30893 + F::cast_from(0.14291339372689912324e-3_f64) * t30901 - t32627 - t32628;
    t37358
}
