//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2142/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2142(t13336: f64, t1898: f64, t249: f64, t23047: f64, t4166: f64, t2635: f64, t81736: f64, t81743: f64, t81750: f64, t87183: f64, t87185: f64, t87187: f64, t87189: f64, t87191: f64, t87193: f64, t87195: f64, t87198: f64, t87200: f64, t87206: f64, t87212: f64, t87213: f64) -> f64 {
    let t87216 = t13336 * t1898 * t249;
    let t87218 = t4166 * t23047;
    let t87219 = t87218 * t2635;
    let t87221 = -t87183 / 768.0_f64 + t87185 / 192.0_f64 + t87187 / 192.0_f64 + t87189 / 192.0_f64 + t87191 / 192.0_f64 - t87193 / 1536.0_f64 - t87195 / 768.0_f64 - t87198 + t87200 / 192.0_f64 - t87206 - t81736 + t81743 - 7.0_f64 / 288.0_f64 * t81750 + t87212 + 0.16821981705891829522e-4_f64 * t87213 + t87216 / 1536.0_f64 + t87219 / 768.0_f64;
    t87221
}
