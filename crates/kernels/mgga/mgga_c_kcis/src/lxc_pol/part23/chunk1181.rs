//! MGGA_C_KCIS lxc pol — lxc_pol part 23 (v4rho3sigma_5) CSE chunk 1181/1323 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part23_v4rho3sigma_5_chunk1181<F: Float>(t2651: F, t7671: F, t26654: F, t838: F, t26633: F, t26652: F, t26420: F, t1505: F, t27489: F, t12286: F, t491: F, t990: F) -> (F, F, F, F, F, F, F) {
    let t93817 = t2651 * t7671;
    let t93826 = t838 * t26654;
    let t93848 = F::cast_from(3.0_f64) * t26633;
    let t93849 = F::cast_from(3.0_f64) * t26652;
    let t93852 = F::cast_from(12.0_f64) * t26420;
    let t94197 = t27489 * t1505;
    let t94208 = t12286 * t491 * t990;
    (t93817, t93826, t93848, t93849, t93852, t94197, t94208)
}
