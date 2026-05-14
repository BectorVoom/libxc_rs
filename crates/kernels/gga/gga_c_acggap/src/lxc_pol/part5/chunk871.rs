//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 871/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk871<F: Float>(t1390: F, t712: F, t1388: F, t1381: F, t2987: F, t2868: F, t484: F, t2970: F, t1268: F, t495: F, t2981: F, t715: F, t288: F, t4027: F, t75: F, t5042: F, t682: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t14959 = t712 * t1390;
    let t14965 = t712 * t1388;
    let t14967 = t1381 * t2987;
    let t14969 = t2868 * t484;
    let t14972 = t2970 * t484;
    let t14974 = t495 * t1268;
    let t14984 = t1381 * t2981;
    let t14986 = t715 * t1390;
    let t14999 = t4027 * t75 * t288;
    let t15003 = t5042 * t682;
    (t14959, t14965, t14967, t14969, t14972, t14974, t14984, t14986, t14999, t15003)
}
