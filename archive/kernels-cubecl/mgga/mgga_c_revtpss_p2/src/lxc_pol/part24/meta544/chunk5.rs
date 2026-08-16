//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1611/1850 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1611<F: Float>(t14791: F, t18426: F, t18627: F, t23160: F, t23334: F, t2745: F, t2747: F, t4362: F, t4364: F, t50370: F, t50372: F, t50377: F, t50381: F, t50385: F, t6017: F, t6035: F, t61570: F, t61572: F, t61576: F, t61623: F, t61645: F, t61675: F, t76284: F, t76289: F, t76321: F, t76428: F) -> F {
    let t87503 = F::cast_from(0.51448821741683684366e-2_f64) * t2745 * t2747 * t18627 * t6017 + F::cast_from(0.77173232612525526552e-2_f64) * t4362 * t4364 * t18426 * t23160 - F::cast_from(0.20579528696673473746e-1_f64) * t4362 * t2747 * t76284 * t23334 + F::cast_from(0.34299214494455789577e-2_f64) * t2745 * t2747 * t76289 * t6035 - F::cast_from(0.20579528696673473746e-1_f64) * t4362 * t14791 * t23160 * t76321 - F::cast_from(0.2032800112371413129e-3_f64) * t76428 - F::cast_from(0.34013387707001991332e-1_f64) * t61570 + F::cast_from(0.81312004494856525159e-3_f64) * t61572 + F::cast_from(0.81312004494856525159e-3_f64) * t61576 + F::cast_from(0.6046824481244798459e0_f64) * t50370 + F::cast_from(0.28900264064772933811e-2_f64) * t50372 - F::cast_from(0.32131292352189751911e-5_f64) * t50377 + F::cast_from(0.45178982497454656791e-6_f64) * t50381 - F::cast_from(0.20553867802866510526e-1_f64) * t50385 - F::cast_from(0.16262400898971305032e-2_f64) * t61623 + F::cast_from(0.36585828794086175548e-2_f64) * t61645 + F::cast_from(0.32524801797942610064e-2_f64) * t61675;
    t87503
}
