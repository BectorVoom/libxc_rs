//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 334/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk334<F: Float>(t538: F, t549: F, t554: F, t118: F, t29: F, t1595: F, t120: F, t1655: F, t528: F, t341: F, t343: F, t72: F, t123: F, t532: F, t126: F, t1631: F, t534: F) -> (F, F, F, F, F, F, F, F) {
    let t2002 = t549 * t538;
    let t2003 = t2002 * t554;
    let t2007 = 1.0 / t118 / t29;
    let t2008 = t2007 * t1595;
    let t2009 = t2008 * t120;
    let t2011 = t528 * t1655;
    let t2012 = t2011 * t120;
    let t2014 = t341 * t343;
    let t2015 = t1595 * t120;
    let t2016 = t72 * t2015;
    let t2021 = t123 / t532 / t29;
    let t2022 = t1595 * t126;
    let t2030 = -0.11705142615505742e0 * t2009 + 0.23410285231011484e0 * t2012 - 0.26564305359272358183e-2 * t2014 * t2016 + 0.319782988780431561e-1 * t2021 * t2022 - 0.532971647967385935e-1 * t534 * t1655 * t126 + 0.13977476158628290272e-1 * t1631 * t2022;
    (t2003, t2007, t2008, t2011, t2014, t2015, t2021, t2030)
}
