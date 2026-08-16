//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 24 (v4rho3sigma_0) CSE chunk 443/1438 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part24_v4rho3sigma_0_chunk443(t2006: f64, t539: f64, t1998: f64, t562: f64, t214: f64, t1985: f64, t553: f64, t544: f64) -> (f64, f64, f64, f64, f64) {
    let t2007 = t539 * t2006;
    let t2009 = t1998 * t562;
    let t2010 = t214 * t2009;
    let t2011 = t1985 * t2010;
    let t2013 = t553 * t2006;
    let t2015 = 0.82246703342411321825e-2_f64 * t2011 + t544 * t2013;
    (t2007, t2009, t2010, t2013, t2015)
}
