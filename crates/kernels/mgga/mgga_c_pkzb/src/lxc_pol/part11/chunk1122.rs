//! MGGA_C_PKZB lxc pol — lxc_pol part 11 (v4rho4_3) CSE chunk 1122/1340 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_pkzb_lxc_pol_part11_v4rho4_3_chunk1122<F: Float>(t23388: F, t411: F, t6546: F, t2363: F, t3246: F, t2393: F, t1448: F, t3308: F, t1435: F, t980: F, t991: F, t1625: F, t3380: F, t83: F) -> (F, F, F, F, F, F, F, F) {
    let t23389 = F::new(0.14291339372689912324e-3) * t23388;
    let t23398 = t411 * t6546;
    let t23465 = t2363 * t3246;
    let t23472 = t2393 * t3246;
    let t23711 = t3308 * t1448;
    let t23796 = t980 * t1435;
    let t23870 = t991 * t1435;
    let t23943 = t83 * t3380 * t1625;
    (t23389, t23398, t23465, t23472, t23711, t23796, t23870, t23943)
}
