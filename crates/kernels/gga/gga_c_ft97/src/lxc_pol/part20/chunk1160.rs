//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1160/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1160<F: Float>(t1882: F, t28278: F, t28273: F, t28264: F, t6907: F, t737: F, t28146: F, t8392: F, t38953: F, t6918: F, t108165: F, t109805: F, t109827: F, t1901: F, t242: F, t2574: F, t2609: F, t265: F, t27878: F, t3821: F, t446: F, t6187: F, t729: F, t762: F, t773: F, t97872: F, t97879: F, t97889: F, t97895: F) -> (F,) {
    let t110931 = 2.0 / 9.0 * t1882 * t28278;
    let t110933 = 2.0 / 9.0 * t1882 * t28273;
    let t110946 = 4.0 / 9.0 * t1882 * t28264;
    let t110950 = t737 * t6907;
    let t110961 = 4.0 / 9.0 * t8392 * t28146;
    let t110962 = t38953 * t6918;
    let t110966 = -8.0 / 27.0 * t97872 - t110931 - t110933 + 2.0 / 3.0 * t446 * t242 * t109827 + 4.0 / 3.0 * t446 * t2574 * t773 * t27878 + 4.0 / 3.0 * t446 * t2574 * t265 * t108165 - t110946 + 2.0 / 3.0 * t446 * t242 * t109805 + 2.0 / 9.0 * t1901 * t110950 * t2609 + t97879 / 9.0 + 2.0 / 3.0 * t446 * t729 * t762 * t6187 * t3821 + t110961 + 4.0 / 81.0 * t110962 + 8.0 / 27.0 * t97889 + t97895 / 9.0;
    (t110966,)
}
