//! GGA_C_FT97 lxc pol — lxc_pol part 11 (v4rho4_0) CSE chunk 374/1030 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part11_v4rho4_0_chunk374<F: Float>(t538: F, t549: F, t554: F, t118: F, t29: F, t1595: F, t120: F, t1655: F, t528: F, t341: F, t343: F) -> (F, F, F, F, F, F, F, F) {
    let t2002 = t549 * t538;
    let t2003 = t2002 * t554;
    let t2007 = 1.0 / t118 / t29;
    let t2008 = t2007 * t1595;
    let t2009 = t2008 * t120;
    let t2011 = t528 * t1655;
    let t2012 = t2011 * t120;
    let t2014 = t341 * t343;
    (t2002, t2003, t2007, t2008, t2009, t2011, t2012, t2014)
}
