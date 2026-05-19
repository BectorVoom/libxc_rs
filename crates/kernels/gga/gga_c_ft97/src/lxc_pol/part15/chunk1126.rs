//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 1126/1222 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk1126<F: Float>(t13605: F, t701: F, t88612: F, t2320: F, t88149: F, t3806: F, t88153: F, t88184: F, t21205: F, t3799: F, t41513: F, t79786: F, t79789: F, t79792: F, t79794: F, t79796: F, t79799: F, t79809: F) -> (F, F, F, F, F, F) {
    let t88614 = t701 * t13605 * t88612;
    let t88617 = t701 * t2320 * t88149;
    let t88621 = t701 * t3806 * t88153;
    let t88624 = t701 * t3806 * t88184;
    let t88626 = t3799 * t21205;
    let t88635 = F::cast_from(0.34049924469135802469e-1_f64) * t88621 - F::cast_from(0.30644932022222222222e0_f64) * t88624 - t41513 + F::cast_from(0.40859909362962962964e0_f64) * t88626 - F::cast_from(0.90799798584362139919e-1_f64) * t79786 + F::cast_from(0.26483274587105624143e-1_f64) * t79789 + F::cast_from(0.51074886703703703704e-1_f64) * t79792 + F::cast_from(0.24969944610699588477e0_f64) * t79794 - F::cast_from(0.68099848938271604939e-1_f64) * t79796 - F::cast_from(0.68099848938271604939e-1_f64) * t79799 - F::cast_from(0.11652640818326474623e1_f64) * t79809;
    (t88614, t88617, t88621, t88624, t88626, t88635)
}
