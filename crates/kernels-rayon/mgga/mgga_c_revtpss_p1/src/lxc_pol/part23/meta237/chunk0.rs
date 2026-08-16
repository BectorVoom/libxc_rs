//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 1389/3317 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk1389(t1390: f64, t6874: f64, t828: f64, t4012: f64, t6836: f64, t124: f64, t6816: f64, t800: f64, t1370: f64, t1388: f64, t1410: f64, t3934: f64, t3976: f64, t3987: f64, t4002: f64, t4064: f64, t5611: f64, t5619: f64, t5623: f64, t6864: f64, t6871: f64) -> (f64, f64, f64, f64, f64) {
    let t6876 = t1390 * t828 * t6874;
    let t6880 = t4012 * t828 * t6836;
    let t6883 = t124 * t6816;
    let t6884 = t800 * t6883;
    let t6887 = -t3976 + t3987 + 0.14291339372689912324e-4_f64 * t5611 + 0.42874018118069736972e-3_f64 * t4002 * t6864 + 0.57165357490759649296e-4_f64 * t5619 - 0.10164000561857065645e-3_f64 * t5623 + 0.17149607247227894789e-2_f64 * t3934 * t6871 - 0.21437009059034868486e-3_f64 * t1388 * t6876 + 0.42874018118069736972e-2_f64 * t1410 * t6880 - t1370 * t6884 / 48.0_f64 - t4064;
    (t6876, t6880, t6883, t6884, t6887)
}
