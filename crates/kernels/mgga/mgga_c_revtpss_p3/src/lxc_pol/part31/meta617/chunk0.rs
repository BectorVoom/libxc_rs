//! MGGA_C_REVTPSS lxc pol — lxc_pol part 31 (v4rho3sigma_6) CSE chunk 2063/2259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part31_v4rho3sigma_6_chunk2063<F: Float>(t10778: F, t1941: F, t25222: F, t4435: F, t14868: F, t2661: F, t93082: F, t14757: F, t25234: F, t14732: F, t25245: F, t14933: F, t2482: F, t25260: F, t814: F) -> (F, F, F, F, F, F) {
    let t99062 = t1941 * t10778;
    let t99066 = t25222 * t4435;
    let t99069 = t2661 * t93082 * t14868;
    let t99070 = F::cast_from(0.57165357490759649296e-4_f64) * t99069;
    let t99073 = t25234 * t14757;
    let t99074 = F::cast_from(0.10164000561857065645e-2_f64) * t99073;
    let t99077 = t25245 * t14732;
    let t99078 = F::cast_from(0.50820002809285328226e-4_f64) * t99077;
    let t99085 = t2482 * t25260 * t814 * t14933;
    (t99062, t99066, t99070, t99074, t99078, t99085)
}
