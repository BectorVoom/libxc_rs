//! MGGA_C_R2SCAN lxc pol — lxc_pol part 17 (v4rho3sigma_7) CSE chunk 570/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part17_v4rho3sigma_7_chunk570<F: Float>(t1856: F, t1858: F, t1874: F, t1885: F, t1888: F, t1897: F, t1901: F, t1904: F, t1910: F, t1913: F, t1916: F, t2037: F, t2789: F, t2795: F, t2800: F) -> F {
    let t3156 = -t1856 + t1858 - F::cast_from(0.10843581300301739842e-1_f64) * t2789 - F::new(2.0) * t2795 - t1874 - t1885 - t1888 - t1897 - t1901 + t1904 + t1910 + t1913 - t1916 + F::cast_from(0.16936279733333333333e-2_f64) * t2800 - t2037;
    t3156
}
