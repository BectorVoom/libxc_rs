//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2299/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2299(t13062: f64, t225: f64, t13378: f64, t10049: f64, t10103: f64, t10110: f64, t10116: f64, t13059: f64, t13377: f64, t1527: f64, t218: f64, t252: f64, t259: f64, t2591: f64, t2710: f64, t2713: f64, t2718: f64, t2719: f64, t4142: f64, t4265: f64, t4268: f64, t4273: f64, t4300: f64, t4301: f64, t46860: f64, t47363: f64, t798: f64, t855: f64, t866: f64, t9590: f64, t9593: f64) -> f64 {
    let t47609 = t13062 * t225;
    let t47618 = t13378 * t225;
    let t47631 = 2.0_f64 * t10103 * t1527 * t2718 * t855 - 18.0_f64 * t10110 * t2719 * t4300 * t855 + 3.0_f64 * t13377 * t259 * t798 + t218 * t259 * t47363 + t252 * t259 * t46860 + 3.0_f64 * t259 * t2591 * t4265 + 3.0_f64 * t259 * t2710 * t4142 + 6.0_f64 * t10049 * t4273 + 6.0_f64 * t10116 * t4268 + 6.0_f64 * t13059 * t2713 - 3.0_f64 * t4301 * t9590 - 6.0_f64 * t4301 * t9593 - 6.0_f64 * t47609 * t866 - 3.0_f64 * t47618 * t866;
    t47631
}
