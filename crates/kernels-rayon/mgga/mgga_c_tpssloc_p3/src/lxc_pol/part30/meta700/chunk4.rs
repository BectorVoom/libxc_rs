//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 30 (v4rho3sigma_6) CSE chunk 2257/2341 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part30_v4rho3sigma_6_chunk2257(t98644: f64, t98688: f64, t98713: f64, t98740: f64, t98795: f64, t98816: f64, t98846: f64, t98873: f64, t25038: f64, t25248: f64, t776: f64, t98422: f64) -> (f64, f64) {
    let t98876 = t98644 + t98688 + t98713 + t98740 + t98795 + t98816 + t98846 + t98873;
    let t98881 = t25038 * t25248 * t98422 * t776;
    (t98876, t98881)
}
