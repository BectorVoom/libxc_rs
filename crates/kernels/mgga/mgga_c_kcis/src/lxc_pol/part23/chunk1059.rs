//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1059/1177 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1059<F: Float>(t2651: F, t7671: F, t26654: F, t838: F, t26633: F, t26652: F, t26420: F, t1505: F, t27489: F, t12286: F, t491: F, t990: F, t3733: F, t27388: F, t4142: F, t27431: F) -> (F, F, F, F, F, F, F, F, F, F) {
    let t93817 = t2651 * t7671;
    let t93826 = t838 * t26654;
    let t93848 = 3.0 * t26633;
    let t93849 = 3.0 * t26652;
    let t93852 = 12.0 * t26420;
    let t94197 = t27489 * t1505;
    let t94208 = t12286 * t491 * t990;
    let t94216 = t3733 * t491;
    let t94223 = t4142 * t27388;
    let t94225 = t4142 * t27431;
    (t93817, t93826, t93848, t93849, t93852, t94197, t94208, t94216, t94223, t94225)
}
