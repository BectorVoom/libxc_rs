//! MGGA_C_REVTPSS kxc pol — kxc_pol part 5 (v3rho3_2) CSE chunk 1387/1422 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_kxc_pol_part5_v3rho3_2_chunk1387<F: Float>(t22125: F, t547: F, t807: F, t4011: F, t6836: F, t1353: F, t6883: F, t800: F, t13832: F, t13851: F, t13858: F, t22107: F, t22111: F, t22115: F, t22120: F, t3934: F, t3944: F, t9739: F, t9742: F, t9766: F) -> F {
    let t22126 = t547 * t22125;
    let t22127 = t807 * t22126;
    let t22129 = t4011 * t6836;
    let t22130 = t547 * t22129;
    let t22131 = t807 * t22130;
    let t22135 = t800 * t6883 * t1353;
    let t22140 = F::new(0.85748036236139473944e-3) * t3934 * t22107 - F::new(0.42874018118069736972e-3) * t3934 * t22111 - F::new(0.21437009059034868486e-3) * t3934 * t22115 - F::new(0.42874018118069736972e-2) * t3934 * t22120 - t13832 + F::new(0.10164000561857065645e-4) * t9739 - F::new(35.0) / F::new(216.0) * t9742 + F::new(0.28582678745379824648e-4) * t22127 - F::new(0.14291339372689912324e-3) * t22131 + F::new(0.50820002809285328224e-4) * t13851 + t3944 * t22135 / F::new(16.0) - F::new(0.90357964994909313582e-5) * t13858 + F::new(0.54208002996571016772e-3) * t9766;
    t22140
}
