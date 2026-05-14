//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1240/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1240<F: Float>(t1299: F, t2316: F, t20: F, t2734: F, t9523: F, t9850: F, t1597: F, t6581: F, t1589: F, t1586: F, t4419: F, t9868: F, t2737: F, t9854: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t33849 = t2316 * t1299;
    let t33850 = t33849 * t20;
    let t33851 = t2734 * t33850;
    let t33854 = t9850 * t9523;
    let t33862 = t1597 * t6581;
    let t33863 = t1589 * t33862;
    let t33864 = t1586 * t33863;
    let t33870 = t4419 * t9868;
    let t33871 = t2737 * t33870;
    let t33873 = t4419 * t9854;
    (t33849, t33850, t33851, t33854, t33862, t33863, t33864, t33870, t33871, t33873)
}
