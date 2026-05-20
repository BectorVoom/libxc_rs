//! MGGA_C_REVTPSS lxc pol — lxc_pol part 36 (v4rho3sigma_11) CSE chunk 694/1378 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part36_v4rho3sigma_11_chunk694<F: Float>(t1390: F, t6874: F, t828: F, t4012: F, t6836: F, t124: F, t6816: F, t800: F, t1370: F, t1388: F, t1410: F, t3934: F, t3976: F, t3987: F, t4002: F, t4064: F, t5611: F, t5619: F, t5623: F, t6864: F, t6871: F) -> (F, F, F, F) {
    let t6876 = t1390 * t828 * t6874;
    let t6880 = t4012 * t828 * t6836;
    let t6883 = t124 * t6816;
    let t6884 = t800 * t6883;
    let t6887 = -t3976 + t3987 + F::cast_from(0.14291339372689912324e-4_f64) * t5611 + F::cast_from(0.42874018118069736972e-3_f64) * t4002 * t6864 + F::cast_from(0.57165357490759649296e-4_f64) * t5619 - F::cast_from(0.10164000561857065645e-3_f64) * t5623 + F::cast_from(0.17149607247227894789e-2_f64) * t3934 * t6871 - F::cast_from(0.21437009059034868486e-3_f64) * t1388 * t6876 + F::cast_from(0.42874018118069736972e-2_f64) * t1410 * t6880 - t1370 * t6884 / F::new(48.0) - t4064;
    (t6876, t6880, t6884, t6887)
}
