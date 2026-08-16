//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1188/1223 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1188(t2030: f64, t20559: f64, t8919: f64, t301: f64, t31146: f64, t4256: f64, t9529: f64, t37791: f64, t37792: f64, t40330: f64, t40332: f64, t40336: f64, t40340: f64, t40344: f64, t40347: f64, t40350: f64, t40354: f64, t40358: f64, t40361: f64, t40365: f64, t40369: f64, t40371: f64, t40374: f64) -> f64 {
    let t40377 = t2030 * t20559 * t8919;
    let t40381 = t31146 * t4256 * t9529 * t301;
    let t40383 = 0.20007875121765877254e-2_f64 * t40330 - 0.40015750243531754507e-2_f64 * t40332 - t37791 - t37792 + 0.68765625e-1_f64 * t40336 + 0.916875e-1_f64 * t40340 - 0.4584375e-1_f64 * t40344 - t40347 / 32.0_f64 - t40350 / 16.0_f64 + 0.916875e-1_f64 * t40354 - 0.916875e-1_f64 * t40358 - 0.4584375e-1_f64 * t40361 - 0.4584375e-1_f64 * t40365 - 0.4584375e-1_f64 * t40369 + 0.16809375e0_f64 * t40371 - 0.916875e-1_f64 * t40374 - 0.4584375e-1_f64 * t40377 + 0.22921875e0_f64 * t40381;
    t40383
}
