//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1278/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1278<F: Float>(t7021: F, t870: F, t1882: F, t29158: F, t29371: F, t29087: F, t8392: F, t10443: F, t112403: F, t14889: F, t1501: F, t15369: F, t15460: F, t15472: F, t1901: F, t2409: F, t24921: F, t24930: F, t25188: F, t2867: F, t2881: F, t2894: F, t29063: F, t29082: F, t29129: F, t296: F, t4246: F, t44538: F, t446: F, t7032: F, t840: F, t871: F, t99665: F) -> (F,) {
    let t114578 = t870 * t7021;
    let t114595 = 2.0 / 9.0 * t1882 * t29158;
    let t114606 = 2.0 / 9.0 * t1882 * t29371;
    let t114616 = 2.0 / 27.0 * t8392 * t29087;
    let t114621 = 4.0 / 3.0 * t446 * t296 * t112403 + t446 * t840 * t4246 * t24921 / 3.0 - 4.0 / 3.0 * t1901 * t15369 * t114578 * t2867 + t1901 * t44538 * t7032 / 9.0 + 2.0 / 9.0 * t1901 * t10443 * t29063 - t446 * t840 * t2894 * t7021 / 3.0 + 8.0 / 81.0 * t99665 + t114595 - 2.0 / 3.0 * t446 * t840 * t25188 * t15472 + t446 * t840 * t871 * t1501 * t14889 / 3.0 - t114606 + 2.0 / 3.0 * t446 * t840 * t4246 * t24930 - 2.0 / 9.0 * t1901 * t2881 * t29082 * t2409 - t114616 + 2.0 * t1901 * t15460 * t29129 * t15472;
    (t114621,)
}
