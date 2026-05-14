//! GGA_C_FT97 lxc pol — lxc_pol part 20 (v4rho3sigma_5) CSE chunk 1030/1293 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part20_v4rho3sigma_5_chunk1030<F: Float>(t6260: F, t880: F, t24964: F, t683: F, t25360: F, t870: F, t1466: F, t25426: F, t681: F, t25491: F, t6210: F, t2842: F, t6347: F, t24900: F, t8392: F, t1882: F, t25324: F) -> (F, F, F, F, F, F, F, F) {
    let t98653 = t6260 * t880;
    let t98694 = t683 * t24964;
    let t98702 = t25360 * t870;
    let t98714 = t1466 * t681 * t25426;
    let t98716 = t6210 * t25491;
    let t98724 = t6347 * t2842;
    let t98738 = t8392 * t24900;
    let t98746 = t1882 * t25324;
    (t98653, t98694, t98702, t98714, t98716, t98724, t98738, t98746)
}
