//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1054/1278 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1054<F: Float>(t35398: F, t35400: F, t35403: F, t35407: F, t35410: F, t35436: F, t35447: F, t35451: F, t35458: F, t35469: F, t35475: F, t35479: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t37528 = F::cast_from(0.68598428988911579156e-2_f64) * t35398;
    let t37529 = F::cast_from(0.10289764348336736873e-1_f64) * t35400;
    let t37531 = F::cast_from(0.34299214494455789578e-2_f64) * t35403;
    let t37533 = t35407 / F::new(16.0);
    let t37534 = t35410 / F::new(48.0);
    let t37551 = F::cast_from(0.16006300097412701803e0_f64) * t35436;
    let t37555 = F::cast_from(0.80031500487063509014e-2_f64) * t35447;
    let t37557 = F::cast_from(0.64025200389650807212e-1_f64) * t35451;
    let t37560 = F::cast_from(0.4528525289702997898e-1_f64) * t35458;
    let t37564 = F::cast_from(0.10289764348336736873e-1_f64) * t35469;
    let t37566 = F::cast_from(0.14291339372689912324e-2_f64) * t35475;
    let t37567 = F::cast_from(0.57165357490759649296e-3_f64) * t35479;
    (t37528, t37529, t37531, t37533, t37534, t37551, t37555, t37557, t37560, t37564, t37566, t37567)
}
