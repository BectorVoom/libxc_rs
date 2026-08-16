//! GGA_C_GAPLOC lxc pol — lxc_pol part 33 (v4rho2sigma2_16) CSE chunk 422/1464 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part33_v4rho2sigma2_16_chunk422(t1966: f64, t1970: f64, t1974: f64, t1977: f64, t1981: f64, t1986: f64, t1988: f64, t1991: f64, t1992: f64, t1995: f64, t1998: f64, t2001: f64, t2004: f64, t2005: f64, t2008: f64, t2009: f64, t2013: f64, t790: f64, t792: f64, t807: f64, t825: f64, t827: f64) -> f64 {
    let t2016 = -0.1022478025437886658e1_f64 * t1966 * t1970 - 0.5680433474654925878e-1_f64 * t825 * t1974 + 0.92686455430723328401e-1_f64 * t790 * t1977 - 0.79445533226334281486e-1_f64 * t1981 * t792 - 0.51123901271894332902e0_f64 * t1986 * t1988 + 0.1022478025437886658e1_f64 * t1991 * t1992 + 0.46011511144704899612e1_f64 * t807 * t1995 - 0.46011511144704899612e1_f64 * t1998 * t2001 + 0.71500979903700853338e0_f64 * t2004 * t2005 - 0.71500979903700853338e0_f64 * t2008 * t2009 + 0.51123901271894332902e0_f64 * t2013 * t827;
    t2016
}
