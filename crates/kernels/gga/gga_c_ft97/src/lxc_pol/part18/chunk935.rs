//! GGA_C_FT97 lxc pol — lxc_pol part 18 (v4rho3sigma_3) CSE chunk 935/1396 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part18_v4rho3sigma_3_chunk935<F: Float>(t1564: F, t3052: F, t5502: F, t15593: F, t2: F, t4: F, t26: F, t376: F, t6422: F, t1286: F, t5743: F, t979: F, t1852: F, t10969: F, t5731: F, t1332: F, t3255: F) -> (F, F, F, F, F, F, F, F, F) {
    let t25579 = t1564 * t5502 * t3052;
    let t25582 = t15593 * t2;
    let t25583 = t25582 * t4;
    let t25584 = t25583 * t26;
    let t25587 = t376 * t6422;
    let t25588 = t1286 * t25587;
    let t25590 = t5743 * t979;
    let t25591 = t1852 * t25590;
    let t25593 = t10969 * t5731;
    let t25595 = t1332 * t3255;
    (t25579, t25583, t25584, t25587, t25588, t25590, t25591, t25593, t25595)
}
