//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 477/1361 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk477(t2003: f64, t2004: f64, t156: f64, t671: f64, t670: f64, t1668: f64, t1675: f64, t1677: f64, t1682: f64, t1685: f64, t1728: f64, t1732: f64, t1737: f64, t1997: f64, t2002: f64) -> (f64, f64, f64, f64) {
    let t2006 = 0.11181742741110338156e-1_f64 * t2003 * t2004;
    let t2007 = t156 * t671;
    let t2009 = 0.72140275749098955847e-1_f64 * t670 * t2007;
    let t2010 = t1668 + 0.21642082724729686754e0_f64 * t1997 + t2002 + t2006 + t2009 - t1675 + t1677 + t1682 - t1685 - t1728 + t1732 + t1737;
    (t2006, t2007, t2009, t2010)
}
