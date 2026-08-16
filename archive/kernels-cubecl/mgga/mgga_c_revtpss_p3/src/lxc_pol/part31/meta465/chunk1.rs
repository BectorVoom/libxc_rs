//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 1707/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk1707<F: Float>(t22125: F, t547: F, t807: F, t4011: F, t6836: F, t1353: F, t6883: F, t800: F, t13832: F, t13851: F, t13858: F, t22107: F, t22111: F, t22115: F, t22120: F, t3934: F, t3944: F, t9739: F, t9742: F, t9766: F) -> (F, F) {
    let t22126 = t547 * t22125;
    let t22127 = t807 * t22126;
    let t22129 = t4011 * t6836;
    let t22130 = t547 * t22129;
    let t22131 = t807 * t22130;
    let t22135 = t800 * t6883 * t1353;
    let t22140 = F::cast_from(0.85748036236139473944e-3_f64) * t3934 * t22107 - F::cast_from(0.42874018118069736972e-3_f64) * t3934 * t22111 - F::cast_from(0.21437009059034868486e-3_f64) * t3934 * t22115 - F::cast_from(0.42874018118069736972e-2_f64) * t3934 * t22120 - t13832 + F::cast_from(0.10164000561857065645e-4_f64) * t9739 - F::cast_from(35.0_f64) / F::cast_from(216.0_f64) * t9742 + F::cast_from(0.28582678745379824648e-4_f64) * t22127 - F::cast_from(0.14291339372689912324e-3_f64) * t22131 + F::cast_from(0.50820002809285328224e-4_f64) * t13851 + t3944 * t22135 / F::cast_from(16.0_f64) - F::cast_from(0.90357964994909313582e-5_f64) * t13858 + F::cast_from(0.54208002996571016772e-3_f64) * t9766;
    (t22129, t22140)
}
