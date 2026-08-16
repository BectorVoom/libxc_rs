//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 22 (v4rho4_3) CSE chunk 1991/2721 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part22_v4rho4_3_chunk1991(t1256: f64, t1763: f64, t19267: f64, t193: f64, t21956: f64, t21958: f64, t21960: f64, t21963: f64, t21990: f64, t22224: f64, t22226: f64, t22231: f64, t22235: f64, t22239: f64, t22241: f64, t22408: f64, t336: f64, t4700: f64) -> f64 {
    let t22412 = t1256 * t193 * t22408 * t336 - 3.0_f64 * t1763 * t19267 * t4700 + t21956 + t21958 + t21960 - t21963 - t21990 - t22224 - t22226 + t22231 - t22235 - t22239 - t22241;
    t22412
}
