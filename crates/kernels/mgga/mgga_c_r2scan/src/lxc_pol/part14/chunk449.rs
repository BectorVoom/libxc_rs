//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 449/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk449<F: Float>(t2008: F, t206: F, t1806: F, t1851: F, t1856: F, t1923: F, t1978: F, t1981: F, t1983: F, t1987: F, t1990: F, t2000: F, t2006: F, t208: F, t226: F, t625: F, t668: F, t682: F, t699: F, t713: F, t718: F) -> (F, F) {
    let t2009 = t206 * t2008;
    let t2013 = -t1806 + F::cast_from(0.17315859105681463759e2_f64) * t718 * t1978 + F::cast_from(0.10254018858216406658e4_f64) * t1981 * t1983 + F::cast_from(0.34631718211362927518e2_f64) * t718 * t1987 + F::cast_from(0.72290542002011598948e-2_f64) * t625 * t1990 * t226 - F::cast_from(0.10843581300301739842e-1_f64) * t625 * t699 * t713 - F::cast_from(0.34246666666666666666e-1_f64) * t625 * t668 * t682 + t1851 - t1856 + F::cast_from(0.22831111111111111111e-1_f64) * t625 * t2000 * t208 + F::cast_from(0.2069040516770936012e4_f64) * t2006 * t2009 * t1923;
    (t2009, t2013)
}
