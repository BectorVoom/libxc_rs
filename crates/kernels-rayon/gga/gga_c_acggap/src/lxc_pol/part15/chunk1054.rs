//! GGA_C_ACGGAP lxc pol — lxc_pol part 15 (v4rho3sigma_7) CSE chunk 1054/1278 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part15_v4rho3sigma_7_chunk1054(t35398: f64, t35400: f64, t35403: f64, t35407: f64, t35410: f64, t35436: f64, t35447: f64, t35451: f64, t35458: f64, t35469: f64, t35475: f64, t35479: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t37528 = 0.68598428988911579156e-2_f64 * t35398;
    let t37529 = 0.10289764348336736873e-1_f64 * t35400;
    let t37531 = 0.34299214494455789578e-2_f64 * t35403;
    let t37533 = t35407 / 16.0_f64;
    let t37534 = t35410 / 48.0_f64;
    let t37551 = 0.16006300097412701803e0_f64 * t35436;
    let t37555 = 0.80031500487063509014e-2_f64 * t35447;
    let t37557 = 0.64025200389650807212e-1_f64 * t35451;
    let t37560 = 0.4528525289702997898e-1_f64 * t35458;
    let t37564 = 0.10289764348336736873e-1_f64 * t35469;
    let t37566 = 0.14291339372689912324e-2_f64 * t35475;
    let t37567 = 0.57165357490759649296e-3_f64 * t35479;
    (t37528, t37529, t37531, t37533, t37534, t37551, t37555, t37557, t37560, t37564, t37566, t37567)
}
