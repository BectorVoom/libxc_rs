//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 7 (v4rho4_0) CSE chunk 627/1242 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part7_v4rho4_0_chunk627<F: Float>(t1815: F, t4882: F, t639: F, t1406: F, t572: F, t418: F, t1821: F, t1820: F, t1866: F, t1827: F, t587: F, t1724: F, t626: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4883 = t1815 * t4882;
    let t4885 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t639 * t4883;
    let t4886 = t1406 * t572;
    let t4887 = t4886 * t418;
    let t4888 = t1821 * t4887;
    let t4890 = F::cast_from(8.0_f64) / F::cast_from(15.0_f64) * t1820 * t4888;
    let t4891 = t1866 * t572;
    let t4892 = t4891 * t418;
    let t4893 = t1827 * t4892;
    let t4895 = F::cast_from(4.0_f64) / F::cast_from(15.0_f64) * t587 * t4893;
    let t4896 = t1724 * t626;
    (t4883, t4885, t4886, t4887, t4888, t4890, t4891, t4892, t4893, t4895, t4896)
}
