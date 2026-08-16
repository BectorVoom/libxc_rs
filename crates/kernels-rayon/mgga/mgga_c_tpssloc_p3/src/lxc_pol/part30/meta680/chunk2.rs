//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2136/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2136(t1992: f64, t550: f64, t57545: f64, t6976: f64, t90750: f64, t90760: f64, t90782: f64, t90789: f64, t90792: f64, t90795: f64, t90798: f64, t90806: f64, t90807: f64, t93517: f64, t96935: f64, t96937: f64, t96941: f64, t96945: f64, t96949: f64, t96954: f64) -> f64 {
    let t96958 = t1992 * t6976 * t57545 * t550;
    let t96960 = 0.3289868133696452873e-1_f64 * t96935 - 0.38381794893125283518e-1_f64 * t96937 - t90750 + t90760 - 0.82246703342411321825e-2_f64 * t96941 + t90782 - 0.49348022005446793095e-1_f64 * t90789 + t90792 + t90795 + t90798 + t90806 - 0.25587863262083522345e0_f64 * t90807 + 0.19190897446562641759e-1_f64 * t96945 - 0.82246703342411321825e-2_f64 * t96949 + 0.49348022005446793095e-1_f64 * t96954 - 0.16449340668482264365e-1_f64 * t96958 - t93517;
    t96960
}
