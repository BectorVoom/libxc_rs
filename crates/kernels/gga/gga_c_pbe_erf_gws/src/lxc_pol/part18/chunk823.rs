//! GGA_C_PBE_ERF_GWS lxc pol — lxc_pol part 18 (v4rho3sigma_6) CSE chunk 823/1210 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_pbe_erf_gws_lxc_pol_part18_v4rho3sigma_6_chunk823<F: Float>(t50: F, t1351: F, t2465: F, t422: F, t52: F, t9801: F, t9993: F, t9998: F, t59: F, t9992: F, t85: F, t4545: F, t6968: F, t8520: F, t7986: F, t4341: F, t4349: F, t4503: F, t4506: F, t4513: F, t4539: F, t4542: F, t6918: F, t6923: F, t6932: F, t7984: F, t9764: F, t9765: F, zeta_threshold: F) -> (F, F, F, F, F) {
    let t51 = t50 <= zeta_threshold;
    let t10004 = piecewise3(t51, 0.0, -8.0 / 27.0 * t9993 * t422 - 16.0 / 9.0 * t2465 * t1351 + 4.0 / 9.0 * t9998 * t422 + 4.0 / 3.0 * t52 * t9801);
    let t10006 = (t9992 + t10004) * t59;
    let t10007 = t10006 * t85;
    let t10008 = 0.19751789702565206229e-1 * t10007;
    let t10009 = 0.63272429661648472106e0 * t4545;
    let t10010 = 0.21687161765563048429e-1 * t6968;
    let t10011 = 0.12654485932329694421e1 * t8520;
    let t10012 = 40.0 * t7986;
    let t10013 = -t9764 + t9765 + t4341 - t4349 - t6918 + t4503 - t4506 - t4513 + t4539 - t6923 + t4542 + t10008 - t10009 + t6932 + t10010 - t7984 - t10011 + t10012;
    (t10006, t10008, t10010, t10012, t10013)
}
