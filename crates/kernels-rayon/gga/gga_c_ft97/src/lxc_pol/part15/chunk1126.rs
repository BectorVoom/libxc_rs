//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1126/1222 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::expint_e1::{xc_e1_scaled};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1126(t13605: f64, t701: f64, t88612: f64, t2320: f64, t88149: f64, t3806: f64, t88153: f64, t88184: f64, t21205: f64, t3799: f64, t41513: f64, t79786: f64, t79789: f64, t79792: f64, t79794: f64, t79796: f64, t79799: f64, t79809: f64) -> (f64, f64, f64, f64, f64, f64) {
    let t88614 = t701 * t13605 * t88612;
    let t88617 = t701 * t2320 * t88149;
    let t88621 = t701 * t3806 * t88153;
    let t88624 = t701 * t3806 * t88184;
    let t88626 = t3799 * t21205;
    let t88635 = 0.34049924469135802469e-1_f64 * t88621 - 0.30644932022222222222e0_f64 * t88624 - t41513 + 0.40859909362962962964e0_f64 * t88626 - 0.90799798584362139919e-1_f64 * t79786 + 0.26483274587105624143e-1_f64 * t79789 + 0.51074886703703703704e-1_f64 * t79792 + 0.24969944610699588477e0_f64 * t79794 - 0.68099848938271604939e-1_f64 * t79796 - 0.68099848938271604939e-1_f64 * t79799 - 0.11652640818326474623e1_f64 * t79809;
    (t88614, t88617, t88621, t88624, t88626, t88635)
}
