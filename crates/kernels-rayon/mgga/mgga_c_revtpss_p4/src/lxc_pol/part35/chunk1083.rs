//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1083/1234 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1083(t5920: f64, t93: f64, t1843: f64, t7983: f64, t5542: f64, t8108: f64, t2097: f64, t6861: f64, t4003: f64, t26079: f64, t26321: f64, t26324: f64, t26325: f64, t26328: f64, t27921: f64, t27926: f64, t27929: f64, t27953: f64, t27955: f64, t30048: f64, t30050: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t30143 = t93 * t5920;
    let t30209 = t1843 * t7983;
    let t30218 = t8108 * t5542;
    let t30225 = t2097 * t6861;
    let t30226 = t30225 * t4003;
    let t30227 = t26079 * t30226;
    let t30246 = t26321 - t26324 - t30048 / 24.0_f64 + 7.0_f64 / 36.0_f64 * t27955 + 0.17149607247227894789e-2_f64 * t30050 + t26325 + t26328 - 0.10164000561857065645e-3_f64 * t27953 + 0.32012600194825403606e-1_f64 * t27926 + 0.57165357490759649296e-4_f64 * t27929 + 0.80031500487063509014e-2_f64 * t27921;
    (t30143, t30209, t30218, t30225, t30226, t30227, t30246)
}
