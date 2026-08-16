//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 19 (v4rho4_0) CSE chunk 1334/1497 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part19_v4rho4_0_chunk1334(t10190: f64, t10255: f64, t2986: f64, t2989: f64, t9258: f64, t10337: f64, t964: f64, t340: f64, t625: f64, t221: f64, t339: f64, t344: f64) -> (f64, f64, f64, f64, f64) {
    let t42794 = t2986 * t10190 * t10255;
    let t42799 = t2989 * t9258;
    let t42811 = t964 * t10337;
    let t42813 = t625 * t340;
    let t42817 = 0.82304526748971193413e-3_f64 * t339 * t221 * t42813 * t344;
    (t42794, t42799, t42811, t42813, t42817)
}
