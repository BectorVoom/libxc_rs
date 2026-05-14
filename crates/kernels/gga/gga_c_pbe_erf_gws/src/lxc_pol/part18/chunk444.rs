//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 444/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk444<F: Float>(t1996: F, t672: F, t225: F, t677: F, t10: F, t670: F, t20: F, t711: F, t245: F, t671: F, t156: F, t226: F, t678: F, t230: F, t666: F, t131: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t1997 = t1996 * t672;
    let t1999 = t225 * t677;
    let t2000 = t10 * t1999;
    let t2002 = 0.21642082724729686754e0 * t670 * t2000;
    let t2003 = t711 * t20;
    let t2004 = t245 * t671;
    let t2006 = 0.11181742741110338156e-1 * t2003 * t2004;
    let t2007 = t156 * t671;
    let t2009 = 0.72140275749098955847e-1 * t670 * t2007;
    let t2014 = 8.0 / 3.0 * t226 * t678;
    let t2015 = t666 * t230;
    let t2029 = t131 * t131;
    (t1997, t1999, t2000, t2002, t2003, t2004, t2006, t2007, t2009, t2014, t2015, t2029)
}
