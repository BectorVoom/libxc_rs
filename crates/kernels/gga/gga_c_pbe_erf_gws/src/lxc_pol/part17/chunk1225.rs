//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 17 (v4rho3sigma_5) CSE chunk 1225/1352 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part17_v4rho3sigma_5_chunk1225<F: Float>(t13796: F, t3989: F, t52921: F, t875: F, t1178: F, t904: F, t14637: F, t9292: F, t14688: F, t2397: F, t14802: F, t2408: F, t29751: F, t50904: F, t52889: F, t52893: F, t52897: F, t52902: F, t52904: F, t52908: F, t52910: F, t52912: F, t52917: F, t52919: F, t827: F) -> (F, F) {
    let t52924 = t3989 * t13796 * t52921 * t875;
    let t52926 = t904 * t1178;
    let t52928 = t14637 * t52926 * t9292;
    let t52930 = t14688 * t2397;
    let t52931 = F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t52930;
    let t52935 = t52889 / F::cast_from(1536.0_f64) + t52893 / F::cast_from(32.0_f64) - t827 * t52897 / F::cast_from(48.0_f64) - F::cast_from(7.0_f64) / F::cast_from(144.0_f64) * t50904 - t52902 + t52904 / F::cast_from(768.0_f64) - t52908 / F::cast_from(192.0_f64) - t52910 / F::cast_from(48.0_f64) - t52912 / F::cast_from(48.0_f64) + t52917 / F::cast_from(192.0_f64) + t52919 / F::cast_from(48.0_f64) - t52924 / F::cast_from(1536.0_f64) - F::cast_from(5.0_f64) / F::cast_from(384.0_f64) * t52928 - t52931 - t2408 * t29751 * t14802 / F::cast_from(12.0_f64);
    (t52926, t52935)
}
