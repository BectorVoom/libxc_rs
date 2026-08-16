//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 29 (v4rho3sigma_5) CSE chunk 2144/2357 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part29_v4rho3sigma_5_chunk2144(t22690: f64, t23171: f64, t25319: f64, t2613: f64, t4291: f64, t7535: f64, t81697: f64, t81704: f64, t81717: f64, t829: f64, t87609: f64, t87613: f64, t87615: f64, t87619: f64, t87620: f64, t87627: f64, t87630: f64, t87633: f64, t87635: f64, t87640: f64, t87645: f64, t87650: f64) -> f64 {
    let t87653 = t23171 * t22690 * t25319;
    let t87656 = 0.16449340668482264365e-1_f64 * t87609 - t87613 + 0.49348022005446793095e-1_f64 * t87615 + t87619 - 2.0_f64 * t4291 * t87620 * t829 + 0.19190897446562641759e-1_f64 * t81697 - 0.82246703342411321825e-2_f64 * t87627 - 0.49348022005446793095e-1_f64 * t87630 + 0.16449340668482264365e-1_f64 * t87633 - 0.12793931631041761173e0_f64 * t87635 + 0.19190897446562641759e-1_f64 * t81704 + 0.49348022005446793095e-1_f64 * t87640 - 0.19739208802178717238e0_f64 * t87645 - 0.16449340668482264365e-1_f64 * t87650 - 0.82246703342411321824e-2_f64 * t87653 + t81717 + t2613 * t7535;
    t87656
}
