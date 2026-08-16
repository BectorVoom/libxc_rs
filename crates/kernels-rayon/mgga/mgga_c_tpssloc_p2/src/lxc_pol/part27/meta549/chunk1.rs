//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 1983/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk1983(t1375: f64, t16460: f64, t2016: f64, t26224: f64, t26226: f64, t26229: f64, t26329: f64, t26335: f64, t26340: f64, t26345: f64, t26348: f64, t26352: f64, t26357: f64, t26361: f64, t3882: f64, t5321: f64, t568: f64, t6963: f64, t7729: f64) -> f64 {
    let t26364 = -6.0_f64 * t26224 * t26226 + t26229 * t568 + t26329 * t568 + 0.49348022005446793095e-1_f64 * t26335 + 0.16449340668482264365e-1_f64 * t26340 + 2.0_f64 * t3882 * t7729 + 0.41123351671205660912e-2_f64 * t26345 + 2.0_f64 * t1375 * t26348 - 0.82246703342411321825e-2_f64 * t26352 + 0.16449340668482264365e-1_f64 * t26357 + 2.0_f64 * t5321 * t6963 - 0.19190897446562641759e-1_f64 * t26361 - t16460 * t2016;
    t26364
}
