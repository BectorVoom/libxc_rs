//! GGA_C_GAPLOC lxc pol — lxc_pol part 24 (v4rho2sigma2_7) CSE chunk 835/1439 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part24_v4rho2sigma2_7_chunk835(t475: f64, t7995: f64, t2343: f64, t447: f64, t7980: f64, t1064: f64, t1305: f64, t2778: f64, t1265: f64, t2787: f64, t1266: f64, t2765: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t7996 = t7995 * t475;
    let t7997 = t2343 * t7996;
    let t8000 = t7980 * t447;
    let t8001 = t1064 * t8000;
    let t8004 = t2778 * t1305;
    let t8005 = t1064 * t8004;
    let t8012 = t2787 * t1265;
    let t8013 = t2343 * t8012;
    let t8016 = t2765 * t1266;
    (t7996, t7997, t8000, t8001, t8004, t8005, t8012, t8013, t8016)
}
