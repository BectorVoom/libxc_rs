//! GGA_C_ACGGAP lxc pol — lxc_pol part 5 (v4rho4_2) CSE chunk 591/1191 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part5_v4rho4_2_chunk591<F: Float>(t381: F, t3832: F, t1004: F, t1241: F, t1248: F, t377: F, t1261: F, t310: F, t1244: F, t460: F, t848: F, t183: F, t3645: F, t1265: F, t857: F, t1210: F, t315: F) -> (F, F, F, F, F, F, F, F, F) {
    let t3833 = t381 * t3832;
    let t3835 = t1004 * t1241;
    let t3837 = t377 * t1248;
    let t3839 = t310 * t1261;
    let t3842 = 0.19756347548806534796e1 * t1004 * t1244;
    let t3843 = t848 * t460;
    let t3846 = 0.65854491829355115987e0 * t3645 * t183;
    let t3856 = t857 * t1265;
    let t3858 = t315 * t1210;
    (t3833, t3835, t3837, t3839, t3842, t3843, t3846, t3856, t3858)
}
