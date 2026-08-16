//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 16 (v4rho3sigma_4) CSE chunk 1307/1361 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part16_v4rho3sigma_4_chunk1307<F: Float>(t14843: F, t804: F, t15097: F, t2053: F, t15081: F, t2376: F, t829: F, t830: F, t52901: F, t1115: F, t4083: F, t50904: F, t52276: F, t52889: F, t52893: F, t52904: F, t52908: F, t52910: F, t52912: F, t52917: F, t52919: F, t52924: F, t52928: F, t827: F, t9265: F) -> (F, F, F) {
    let t54866 = F::cast_from(6.0_f64) * t804 * t14843;
    let t54867 = t15097 * t2053;
    let t54880 = t2376 * t15081;
    let t54882 = t829 * t830 * t54880;
    let t54886 = F::cast_from(7.0_f64) / F::cast_from(576.0_f64) * t52901;
    let t54895 = t52889 / F::cast_from(768.0_f64) + t52893 / F::cast_from(16.0_f64) - t9265 * t4083 / F::cast_from(96.0_f64) - t1115 * t52276 / F::cast_from(48.0_f64) - t827 * t54882 / F::cast_from(48.0_f64) - F::cast_from(7.0_f64) / F::cast_from(72.0_f64) * t50904 - t54886 + t52904 / F::cast_from(384.0_f64) - t52908 / F::cast_from(96.0_f64) - t52910 / F::cast_from(24.0_f64) - t52912 / F::cast_from(24.0_f64) + t52917 / F::cast_from(96.0_f64) + t52919 / F::cast_from(24.0_f64) - t52924 / F::cast_from(768.0_f64) - F::cast_from(5.0_f64) / F::cast_from(192.0_f64) * t52928;
    (t54866, t54867, t54895)
}
