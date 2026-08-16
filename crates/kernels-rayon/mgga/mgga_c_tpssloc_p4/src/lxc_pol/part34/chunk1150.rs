//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 34 (v4rho3sigma_10) CSE chunk 1150/1250 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part34_v4rho3sigma_10_chunk1150(t23062: f64, t28383: f64, t5568: f64, t81956: f64, t28389: f64, t81963: f64, t23083: f64, t28356: f64, t23133: f64, t5628: f64, t23041: f64, t5614: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t98696 = t23062 * t28383;
    let t98709 = t81956 * t5568;
    let t98711 = t81963 * t28389;
    let t98725 = t23083 * t28356;
    let t98733 = t23133 * t5628;
    let t98736 = t23041 * t5614;
    (t98696, t98709, t98711, t98725, t98733, t98736)
}
