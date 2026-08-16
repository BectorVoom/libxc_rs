//! MGGA_C_TPSSLOC lxc pol kernel — _part31_v4rho3sigma_7 meta604 (260520-c91 hierarchical CSE).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

mod chunk0;

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

use chunk0::mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1849;

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_meta604(t225: f64, t26732: f64, t87776: f64, t87786: f64, t87796: f64, t87804: f64, t87835: f64, t87873: f64, t26734: f64, t87901: f64, t87910: f64, t87927: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64, f64, f64) {
        let (t92847, t92862, t92866, t92872, t92874, t92910, t92938, t92939, t92955, t92960, t92966) = mgga_c_tpssloc_lxc_pol_part31_v4rho3sigma_7_chunk1849(t225, t26732, t87776, t87786, t87796, t87804, t87835, t87873, t26734, t87901, t87910, t87927);
    (t92847, t92862, t92866, t92872, t92874, t92910, t92938, t92939, t92955, t92960, t92966)
}
