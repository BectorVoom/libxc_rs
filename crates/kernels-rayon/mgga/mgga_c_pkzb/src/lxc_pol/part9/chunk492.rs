//! MGGA_C_PKZB lxc pol — lxc_pol part 9 (v4rho4_1) CSE chunk 492/1336 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_pkzb_lxc_pol_part9_v4rho4_1_chunk492(t2003: f64, t66: f64, t179: f64, t1885: f64, t1849: f64, t1852: f64, t1859: f64, t1896: f64, t1904: f64, t1984: f64, t1986: f64, t1989: f64, t1993: f64, t1997: f64, t2001: f64) -> (f64, f64) {
    let t2004 = t66 * t2003;
    let t2006 = t179 * t2004 * t1885;
    let t2009 = -t1849 + t1852 - t1859 + t1896 + t1904 + t1984 + t1986 - t1989 + t1993 - t1997 - t2001;
    (t2006, t2009)
}
