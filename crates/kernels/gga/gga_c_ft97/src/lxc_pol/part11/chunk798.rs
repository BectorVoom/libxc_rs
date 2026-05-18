//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 798/1173 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk798<F: Float>(t10799: F, t871: F, t296: F, t1882: F, t2846: F, t10735: F, t10738: F, t10741: F, t10745: F, t10749: F, t10750: F, t10752: F, t10755: F, t10760: F, t10765: F, t10769: F, t10771: F, t10773: F, t446: F) -> (F, F, F) {
    let t10800 = t871 * t10799;
    let t10801 = t296 * t10800;
    let t10804 = t1882 * t2846;
    let t10806 = -F::new(4.0) / F::new(9.0) * t10735 - t446 * t10738 - t446 * t10741 / F::new(3.0) - t10745 / F::new(3.0) - t10749 + t10750 / F::new(3.0) + F::new(2.0) / F::new(3.0) * t10752 + F::new(2.0) * t446 * t10755 - F::new(10.0) / F::new(81.0) * t446 * t10760 - F::new(2.0) * t446 * t10765 - t446 * t10769 + t10771 / F::new(3.0) - F::new(4.0) / F::new(9.0) * t10773 - t446 * t10801 / F::new(3.0) - F::new(2.0) / F::new(3.0) * t10804;
    (t10800, t10801, t10806)
}
