//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 48 (v4rho2sigma2_4) CSE chunk 605/1034 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part48_v4rho2sigma2_4_chunk605(t6899: f64, t1323: f64, t2085: f64, t6914: f64, t6921: f64, t6934: f64, t6948: f64, t6917: f64, t6929: f64, t6938: f64, t6941: f64, t6946: f64, t6953: f64) -> (f64, f64, f64) {
    let t7176 = 0.82246703342411321825e-2_f64 * t6899;
    let t7179 = t1323 * t2085;
    let t7181 = 7.0_f64 / 144.0_f64 * t6914;
    let t7183 = 0.28260929265898273597e-2_f64 * t6921;
    let t7185 = 0.67287926823567318088e-4_f64 * t6934;
    let t7189 = 7.0_f64 / 1152.0_f64 * t6948;
    let t7191 = -t7181 - t6917 / 24.0_f64 - t7183 - 0.24223653656484234512e-2_f64 * t6929 - t7185 - 0.40372756094140390853e-3_f64 * t6938 + t6941 / 768.0_f64 - t6946 / 768.0_f64 - t7189 - t6953 / 192.0_f64;
    (t7176, t7179, t7191)
}
