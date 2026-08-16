//! MGGA_C_REVTPSS lxc pol — lxc_pol part 20 (v4rho4_0) CSE chunk 1324/1798 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part20_v4rho4_0_chunk1324(t45: f64, t2495: f64, t9385: f64, t2491: f64, t744: f64, t760: f64, t2492: f64, t2514: f64, t9367: f64, t9371: f64, t200: f64, t631: f64, t10326: f64, t10446: f64, t10449: f64, t2251: f64, t2258: f64, t2375: f64, t39443: f64, t39449: f64, t39457: f64, t78: f64, zeta_threshold: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t151 = t45 <= zeta_threshold;
    let t39815 = t2495 * t9385;
    let t39816 = t2491 * t744 * t39815;
    let t39818 = 0.69263436422725855036e2_f64 * t760 * t39816;
    let t39821 = t9367 * t2492 * t9371 * t2514;
    let t39823 = 0.61524113149298439947e4_f64 * t760 * t39821;
    let t39825 = 1.0_f64 / t200 / t631;
    let t39838 = piecewise3(t151, 0.0_f64, 40.0_f64 / 81.0_f64 * t39825 * t39443 - 16.0_f64 / 9.0_f64 * t10446 * t2251 * t2258 + 4.0_f64 / 3.0_f64 * t2375 * t39449 + 16.0_f64 / 9.0_f64 * t10449 * t10326 + 4.0_f64 / 3.0_f64 * t78 * t39457);
    (t39815, t39816, t39818, t39821, t39823, t39838)
}
