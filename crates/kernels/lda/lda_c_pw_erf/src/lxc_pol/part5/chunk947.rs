//! LDA_C_PW_ERF lxc pol — lxc_pol part 5 (v4rho4_3) CSE chunk 947/1157 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI, M_SQRT2};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};
use libxc_kernel_math::piecewise::{piecewise3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn lda_c_pw_erf_lxc_pol_part5_v4rho4_3_chunk947<F: Float>(t8528: F, t14445: F, t405: F, t7976: F, t2765: F, t7970: F, t10832: F, t11499: F, t11501: F, t14491: F, t1735: F, t1808: F, t1809: F, t1859: F, t18795: F, t19425: F, t19832: F, t2211: F, t2591: F, t2610: F, t440: F, t5495: F, t5783: F, t5924: F, t6025: F, t6121: F, t6154: F, t7082: F, t770: F, t777: F, t7880: F, t7889: F, t7986: F, t7987: F, t8751: F, t9156: F) -> (F, F, F) {
    let t20097 = 60.0 * t8528;
    let t20098 = 1.7544670192365612 * t14445;
    let t20106 = t405 * t7976;
    let t20120 = t2765 * t7970;
    let t20138 = -6.0 * t19832 * t2765 * t7986 * t440 + 36.0 * t6025 * t19425 + 0.11974234010254609 * t18795 + 3.0 * t20106 * t1735 + 9.0 * t2211 * t2591 * t5495 + 9.0 * t2211 * t7082 * t1809 - 9.0 * t5783 * t10832 * t7880 - t11499 - 0.002615106736609823 * t11501 - 0.9247854820715865 * t8751 + 18.0 * t5924 * t20120 + 18.0 * t14491 * t7889 + 18.0 * t1808 * t5495 * t2610 + 18.0 * t1808 * t1809 * t6121 + 2.0 * t777 * t9156 * t7987 + 4.0 * t6154 * t2765 * t770 * t1859;
    (t20097, t20098, t20138)
}
