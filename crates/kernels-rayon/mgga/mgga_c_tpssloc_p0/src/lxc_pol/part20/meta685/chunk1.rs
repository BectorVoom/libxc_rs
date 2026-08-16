//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 20 (v4rho4_1) CSE chunk 2596/2712 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part20_v4rho4_1_chunk2596(t3610: f64, t52627: f64, t11154: f64, t11668: f64, t11680: f64, t11688: f64, t11825: f64, t11863: f64, t1227: f64, t15453: f64, t15569: f64, t1735: f64, t3577: f64, t3580: f64, t44996: f64, t4582: f64, t48554: f64, t4954: f64, t4989: f64, t5024: f64, t52610: f64, t52615: f64, t52619: f64, t52621: f64) -> f64 {
    let t52628 = t3610 * t52627;
    let t52639 = -t52610 - 5.0_f64 / 1728.0_f64 * t1227 * t4582 * t15453 * t48554 + t52615 * t3580 / 144.0_f64 - t52619 / 2304.0_f64 - t52621 / 1152.0_f64 + t15569 * t11688 / 144.0_f64 - t44996 * t4954 / 1536.0_f64 + t52628 * t11680 / 144.0_f64 + 5.0_f64 / 2304.0_f64 * t3577 * t11668 * t1735 * t11154 + 5.0_f64 / 4608.0_f64 * t11825 * t4989 + t5024 * t11863 / 144.0_f64;
    t52639
}
