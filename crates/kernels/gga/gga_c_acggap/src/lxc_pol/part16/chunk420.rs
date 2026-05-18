//! GGA_C_ACGGAP lxc pol — lxc_pol part 16 (v4rho3sigma_8) CSE chunk 420/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part16_v4rho3sigma_8_chunk420<F: Float>(t2001: F, t425: F, t431: F, t438: F, t413: F, t587: F, t151: F, t177: F, t377: F, t588: F, t130: F, t163: F) -> (F, F, F, F, F, F, F, F, F) {
    let t2002 = t2001 * t425;
    let t2004 = t2001 * t431;
    let t2006 = t2001 * t438;
    let t2008 = t587 * t413;
    let t2009 = t151 * t2008;
    let t2010 = t2009 * t177;
    let t2011 = F::new(0.20007875121765877254e-2) * t2010;
    let t2012 = t377 * t588;
    let t2013 = t2012 * t177;
    let t2014 = F::new(0.42874018118069736972e-3) * t2013;
    let t2015 = t130 * t163;
    (t2002, t2004, t2006, t2008, t2009, t2011, t2012, t2014, t2015)
}
