//! GGA_C_GAPLOC lxc pol — lxc_pol part 18 (v4rho2sigma2_1) CSE chunk 1269/1436 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part18_v4rho2sigma2_1_chunk1269(t28099: f64, t11019: f64, t1445: f64, t1998: f64, t32190: f64, t32875: f64, t32878: f64, t32881: f64, t32884: f64, t32886: f64, t32888: f64, t32892: f64, t32896: f64, t32900: f64, t32902: f64, t32904: f64, t32907: f64, t32910: f64, t4614: f64, t701: f64) -> f64 {
    let t32911 = 0.15976219147466979032e-1_f64 * t28099;
    let t32919 = -t32875 - t32878 - t32881 + t32884 - t32886 + t32888 + t32892 + t32896 + t32900 + t32902 + t32904 - t32907 + t32910 - t32911 - 0.46011511144704899612e1_f64 * t1998 * t1445 * t32190 * t701 - 0.61348681526273199482e1_f64 * t1998 * t4614 * t11019;
    t32919
}
