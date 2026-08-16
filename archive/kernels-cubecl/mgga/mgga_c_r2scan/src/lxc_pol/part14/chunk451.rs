//! MGGA_C_R2SCAN lxc pol — lxc_pol part 14 (v4rho3sigma_4) CSE chunk 451/1276 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part14_v4rho3sigma_4_chunk451<F: Float>(t1936: F, t1976: F, t2013: F, t2033: F, t61: F, t41: F, t1856: F, t1858: F, t1863: F, t1866: F, t1874: F, t1875: F, t1885: F, t1888: F, t1897: F, t1901: F, t1904: F, t1910: F, t1913: F, t1916: F) -> (F, F, F, F) {
    let t2035 = t1936 + t1976 + t2013 + t2033;
    let t2036 = t61 * t2035;
    let t2037 = t41 * t2036;
    let t2038 = -t1856 - t1858 + F::cast_from(0.2701041328e0_f64) * t1863 - F::cast_from(0.1143056e0_f64) * t1866 - t1874 - F::cast_from(8.0_f64) * t1875 - t1885 - t1888 - t1897 - t1901 + t1904 + t1910 + t1913 - t1916 - t2037;
    (t2035, t2036, t2037, t2038)
}
