//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 610/1100 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk610<F: Float>(t418: F, t4886: F, t1821: F, t1820: F, t1866: F, t572: F, t1827: F, t587: F, t1724: F, t626: F, t422: F, t1815: F, t639: F, t1733: F, t1809: F, t1620: F) -> (F, F, F, F, F, F, F, F, F, F, F, F, F, F, F) {
    let t4887 = t4886 * t418;
    let t4888 = t1821 * t4887;
    let t4890 = 8.0 / 15.0 * t1820 * t4888;
    let t4891 = t1866 * t572;
    let t4892 = t4891 * t418;
    let t4893 = t1827 * t4892;
    let t4895 = 4.0 / 15.0 * t587 * t4893;
    let t4896 = t1724 * t626;
    let t4897 = t4896 * t422;
    let t4898 = t1815 * t4897;
    let t4900 = 4.0 / 15.0 * t639 * t4898;
    let t4901 = t1733 * t626;
    let t4902 = t4901 * t422;
    let t4903 = t1809 * t4902;
    let t4905 = 8.0 / 15.0 * t1620 * t4903;
    (t4887, t4888, t4890, t4891, t4892, t4893, t4895, t4896, t4897, t4898, t4900, t4901, t4902, t4903, t4905)
}
