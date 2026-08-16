//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 27 (v4rho3sigma_3) CSE chunk 2317/2372 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part27_v4rho3sigma_3_chunk2317(t225: f64, t3787: f64, t562: f64, t16313: f64, t91004: f64, t22751: f64, t26385: f64, t16068: f64, t1992: f64, t6976: f64, t81149: f64, t16060: f64, t26403: f64, t3856: f64, t5250: f64, t5334: f64, t5344: f64, t6988: f64, t81115: f64, t81125: f64, t81127: f64, t81140: f64, t81147: f64, t81154: f64, t90942: f64, t90988: f64, t90993: f64, t91000: f64, t91002: f64) -> (f64, f64) {
    let t91005 = t225 * t3787;
    let t91006 = t91005 * t562;
    let t91008 = t91004 * t91006 * t16313;
    let t91010 = t22751 * t26385;
    let t91011 = 0.76763589786250567036e-1_f64 * t91010;
    let t91014 = t1992 * t6976 * t16068;
    let t91018 = 0.16449340668482264365e-1_f64 * t81149;
    let t91019 = -t90988 + 4.0_f64 * t5334 * t90942 * t5250 - 0.82246703342411321824e-2_f64 * t90993 + 0.41123351671205660912e-2_f64 * t81115 - t5344 * t26403 * t3856 + 0.41123351671205660912e-2_f64 * t81125 + 0.38381794893125283518e-1_f64 * t81127 - 0.63969658155208805863e-1_f64 * t91000 - 0.2302907693587517011e0_f64 * t91002 - 0.6579736267392905746e-1_f64 * t91008 + t91011 - 0.24674011002723396547e-1_f64 * t81140 - t81147 - 0.16449340668482264365e-1_f64 * t91014 - 2.0_f64 * t16060 * t6988 - t91018 + t81154;
    (t91005, t91019)
}
