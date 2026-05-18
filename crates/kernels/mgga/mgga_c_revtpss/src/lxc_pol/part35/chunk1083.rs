//! MGGA_C_REVTPSS lxc pol — lxc_pol part 35 (v4rho3sigma_10) CSE chunk 1083/1234 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part35_v4rho3sigma_10_chunk1083<F: Float>(t5920: F, t93: F, t1843: F, t7983: F, t5542: F, t8108: F, t2097: F, t6861: F, t4003: F, t26079: F, t26321: F, t26324: F, t26325: F, t26328: F, t27921: F, t27926: F, t27929: F, t27953: F, t27955: F, t30048: F, t30050: F) -> (F, F, F, F, F, F, F) {
    let t30143 = t93 * t5920;
    let t30209 = t1843 * t7983;
    let t30218 = t8108 * t5542;
    let t30225 = t2097 * t6861;
    let t30226 = t30225 * t4003;
    let t30227 = t26079 * t30226;
    let t30246 = t26321 - t26324 - t30048 / F::new(24.0) + F::new(7.0) / F::new(36.0) * t27955 + F::new(0.17149607247227894789e-2) * t30050 + t26325 + t26328 - F::new(0.10164000561857065645e-3) * t27953 + F::new(0.32012600194825403606e-1) * t27926 + F::new(0.57165357490759649296e-4) * t27929 + F::new(0.80031500487063509014e-2) * t27921;
    (t30143, t30209, t30218, t30225, t30226, t30227, t30246)
}
