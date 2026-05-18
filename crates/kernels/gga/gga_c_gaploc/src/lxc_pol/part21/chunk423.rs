//! GGA_C_GAPLOC lxc pol — lxc_pol part 21 (v4rho2sigma2_4) CSE chunk 423/1466 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part21_v4rho2sigma2_4_chunk423<F: Float>(t1966: F, t1970: F, t1974: F, t1977: F, t1981: F, t1986: F, t1988: F, t1991: F, t1992: F, t1995: F, t1998: F, t2001: F, t2004: F, t2005: F, t2008: F, t2009: F, t2013: F, t790: F, t792: F, t807: F, t825: F, t827: F) -> F {
    let t2016 = -F::new(0.1022478025437886658e1) * t1966 * t1970 - F::new(0.5680433474654925878e-1) * t825 * t1974 + F::new(0.92686455430723328401e-1) * t790 * t1977 - F::new(0.79445533226334281486e-1) * t1981 * t792 - F::new(0.51123901271894332902e0) * t1986 * t1988 + F::new(0.1022478025437886658e1) * t1991 * t1992 + F::new(0.46011511144704899612e1) * t807 * t1995 - F::new(0.46011511144704899612e1) * t1998 * t2001 + F::new(0.71500979903700853338e0) * t2004 * t2005 - F::new(0.71500979903700853338e0) * t2008 * t2009 + F::new(0.51123901271894332902e0) * t2013 * t827;
    t2016
}
