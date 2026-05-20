//! MGGA_C_REVTPSS lxc pol — lxc_pol part 23 (v4rho4_3) CSE chunk 3031/3317 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part23_v4rho4_3_chunk3031<F: Float>(t1043: F, t16410: F, t1647: F, t16520: F, t16544: F, t16553: F, t19484: F, t19498: F, t19569: F, t19602: F, t19607: F, t24090: F, t24098: F, t3223: F, t3317: F, t3318: F, t357: F, t43350: F, t43520: F, t4984: F, t4995: F, t4996: F, t4998: F, t4999: F, t55805: F, t55938: F, t55939: F, t6235: F, t78496: F, t78812: F, t78873: F, t80640: F, t999: F) -> F {
    let t80764 = -F::cast_from(0.19756347548806534796e1_f64) * t6235 * t4995 * t4999 - F::cast_from(0.19756347548806534796e1_f64) * t3223 * t24098 - F::cast_from(0.19756347548806534796e1_f64) * t3317 * t80640 * t3318 + F::cast_from(0.92196288561097162379e1_f64) * t55938 * t78812 * t55939 * t1043 - F::cast_from(0.65854491829355115987e0_f64) * t55805 * t78812 * t43350 * t1043 * t357 - F::cast_from(0.39512695097613069591e1_f64) * t43520 * t78496 * t16553 * t999 + F::cast_from(0.39512695097613069591e1_f64) * t16410 * t24090 + F::cast_from(0.79025390195226139182e1_f64) * t1647 * t19602 * t4984 - F::cast_from(0.39512695097613069591e1_f64) * t1647 * t19607 * t4999 + F::cast_from(0.39512695097613069591e1_f64) * t16520 * t24090 - F::cast_from(0.39512695097613069592e1_f64) * t19569 * t19484 - F::cast_from(0.19756347548806534796e1_f64) * t16544 * t19498 - F::cast_from(0.65854491829355115987e0_f64) * t4996 * t78873 * t4998;
    t80764
}
