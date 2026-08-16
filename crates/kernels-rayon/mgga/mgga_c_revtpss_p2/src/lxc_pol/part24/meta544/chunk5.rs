//! MGGA_C_REVTPSS lxc pol — lxc_pol part 24 (v4rho4_4) CSE chunk 1611/1850 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_revtpss_lxc_pol_part24_v4rho4_4_chunk1611(t14791: f64, t18426: f64, t18627: f64, t23160: f64, t23334: f64, t2745: f64, t2747: f64, t4362: f64, t4364: f64, t50370: f64, t50372: f64, t50377: f64, t50381: f64, t50385: f64, t6017: f64, t6035: f64, t61570: f64, t61572: f64, t61576: f64, t61623: f64, t61645: f64, t61675: f64, t76284: f64, t76289: f64, t76321: f64, t76428: f64) -> f64 {
    let t87503 = 0.51448821741683684366e-2_f64 * t2745 * t2747 * t18627 * t6017 + 0.77173232612525526552e-2_f64 * t4362 * t4364 * t18426 * t23160 - 0.20579528696673473746e-1_f64 * t4362 * t2747 * t76284 * t23334 + 0.34299214494455789577e-2_f64 * t2745 * t2747 * t76289 * t6035 - 0.20579528696673473746e-1_f64 * t4362 * t14791 * t23160 * t76321 - 0.2032800112371413129e-3_f64 * t76428 - 0.34013387707001991332e-1_f64 * t61570 + 0.81312004494856525159e-3_f64 * t61572 + 0.81312004494856525159e-3_f64 * t61576 + 0.6046824481244798459e0_f64 * t50370 + 0.28900264064772933811e-2_f64 * t50372 - 0.32131292352189751911e-5_f64 * t50377 + 0.45178982497454656791e-6_f64 * t50381 - 0.20553867802866510526e-1_f64 * t50385 - 0.16262400898971305032e-2_f64 * t61623 + 0.36585828794086175548e-2_f64 * t61645 + 0.32524801797942610064e-2_f64 * t61675;
    t87503
}
