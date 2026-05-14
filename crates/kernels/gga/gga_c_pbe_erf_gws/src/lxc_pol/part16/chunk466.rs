//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 466/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk466<F: Float>(t225: F, t677: F, t10: F, t670: F, t20: F, t711: F, t245: F, t671: F, t156: F, t1668: F, t1675: F, t1677: F, t1682: F, t1685: F, t1728: F, t1732: F, t1737: F, t1997: F) -> (F, F, F, F, F, F, F, F, F) {
    let t1999 = t225 * t677;
    let t2000 = t10 * t1999;
    let t2002 = 0.21642082724729686754e0 * t670 * t2000;
    let t2003 = t711 * t20;
    let t2004 = t245 * t671;
    let t2006 = 0.11181742741110338156e-1 * t2003 * t2004;
    let t2007 = t156 * t671;
    let t2009 = 0.72140275749098955847e-1 * t670 * t2007;
    let t2010 = t1668 + 0.21642082724729686754e0 * t1997 + t2002 + t2006 + t2009 - t1675 + t1677 + t1682 - t1685 - t1728 + t1732 + t1737;
    (t1999, t2000, t2002, t2003, t2004, t2006, t2007, t2009, t2010)
}
