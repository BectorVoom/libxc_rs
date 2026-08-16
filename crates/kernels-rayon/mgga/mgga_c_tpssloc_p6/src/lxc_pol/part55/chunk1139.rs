//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 55 (v4rho2sigma2_11) CSE chunk 1139/1304 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part55_v4rho2sigma2_11_chunk1139(t112: f64, t27907: f64, t111: f64, t8110: f64, t1307: f64, t1842: f64, t1527: f64, t776: f64, t671: f64, t7982: f64, t2169: f64, t214: f64, t6624: f64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let t96311 = t27907 * t112;
    let t96334 = t8110 * t111;
    let t97721 = t1842 * t1307;
    let t98960 = t1527 * t776;
    let t104977 = t7982 * t671;
    let t105108 = t2169 * t671;
    let t112660 = t214 * t6624;
    (t96311, t96334, t97721, t98960, t104977, t105108, t112660)
}
