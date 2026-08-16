//! GGA_C_FT97 lxc pol — lxc_pol part 2 (v3rho3_1) CSE chunk 814/1007 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part2_v3rho3_1_chunk814(t12313: f64, t2102: f64, t12317: f64, t9217: f64, t11050: f64, t3506: f64, t11059: f64, t3499: f64, t1017: f64, t2112: f64, t1970: f64, t1570: f64, t586: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t12775 = t2102 * t12313;
    let t12778 = t9217 * t12317;
    let t12781 = t3506 * t11050;
    let t12784 = t3499 * t11059;
    let t12787 = t2112 * t1017;
    let t12788 = t12787 * t1970;
    let t12791 = t586 * t1570;
    (t12775, t12778, t12781, t12784, t12788, t12791)
}
