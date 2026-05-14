//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 447/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk447<F: Float>(t245: F, t671: F, t2003: F, t156: F, t670: F, t1668: F, t1675: F, t1677: F, t1682: F, t1685: F, t1728: F, t1732: F, t1737: F, t1997: F, t2002: F, t1739: F, t1742: F, t1752: F, t1777: F, t1780: F, t1785: F, t1789: F, t1797: F, t1800: F, t1808: F, t1814: F, t1819: F) -> (F, F, F, F) {
    let t2004 = t245 * t671;
    let t2006 = 0.11181742741110338156e-1 * t2003 * t2004;
    let t2007 = t156 * t671;
    let t2009 = 0.72140275749098955847e-1 * t670 * t2007;
    let t2010 = t1668 + 0.21642082724729686754e0 * t1997 + t2002 + t2006 + t2009 - t1675 + t1677 + t1682 - t1685 - t1728 + t1732 + t1737;
    let t2012 = -t1739 - t1742 + t1752 + t1777 - t1780 + t1785 + t1789 + t1797 + t1800 + t1808 + t1814 - t1819;
    (t2004, t2007, t2010, t2012)
}
