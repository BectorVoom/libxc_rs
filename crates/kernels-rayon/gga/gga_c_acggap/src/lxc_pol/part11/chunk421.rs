//! GGA_C_ACGGAP lxc pol — lxc_pol part 11 (v4rho3sigma_3) CSE chunk 421/1213 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_acggap_lxc_pol_part11_v4rho3sigma_3_chunk421(t2001: f64, t425: f64, t431: f64, t438: f64, t413: f64, t587: f64, t151: f64, t177: f64, t377: f64, t588: f64, t130: f64, t163: f64) -> (f64, f64, f64, f64, f64, f64, f64, f64, f64) {
    let t2002 = t2001 * t425;
    let t2004 = t2001 * t431;
    let t2006 = t2001 * t438;
    let t2008 = t587 * t413;
    let t2009 = t151 * t2008;
    let t2010 = t2009 * t177;
    let t2011 = 0.20007875121765877254e-2_f64 * t2010;
    let t2012 = t377 * t588;
    let t2013 = t2012 * t177;
    let t2014 = 0.42874018118069736972e-3_f64 * t2013;
    let t2015 = t130 * t163;
    (t2002, t2004, t2006, t2008, t2009, t2011, t2012, t2014, t2015)
}
