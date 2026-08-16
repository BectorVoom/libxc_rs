//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 419/1264 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk419(t2008: f64, t206: f64, t1806: f64, t1851: f64, t1856: f64, t1923: f64, t1978: f64, t1981: f64, t1983: f64, t1987: f64, t1990: f64, t2000: f64, t2006: f64, t208: f64, t226: f64, t625: f64, t668: f64, t682: f64, t699: f64, t713: f64, t718: f64) -> (f64, f64) {
    let t2009 = t206 * t2008;
    let t2013 = -t1806 + 0.17315859105681463759e2_f64 * t718 * t1978 + 0.10254018858216406658e4_f64 * t1981 * t1983 + 0.34631718211362927518e2_f64 * t718 * t1987 + 0.72290542002011598948e-2_f64 * t625 * t1990 * t226 - 0.10843581300301739842e-1_f64 * t625 * t699 * t713 - 0.34246666666666666666e-1_f64 * t625 * t668 * t682 + t1851 - t1856 + 0.22831111111111111111e-1_f64 * t625 * t2000 * t208 + 0.2069040516770936012e4_f64 * t2006 * t2009 * t1923;
    (t2009, t2013)
}
