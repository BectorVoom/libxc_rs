//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1016/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1016<F: Float>(t35844: F, t7447: F, t8823: F, t7440: F, t8826: F, t30817: F, t8948: F, t8793: F, t1313: F, t30598: F, t721: F, t1322: F, t7859: F) -> (F, F, F, F, F, F, F) {
    let t35845 = F::cast_from(0.21437009059034868486e-3_f64) * t35844;
    let t35848 = t7447 * t8823;
    let t35849 = F::new(0.84046875e-1) * t35848;
    let t35850 = t7440 * t8826;
    let t35851 = F::new(0.5603125e-1) * t35850;
    let t35874 = t30817 * t8948;
    let t35875 = F::cast_from(0.25724410870841842184e-2_f64) * t35874;
    let t35876 = t30817 * t8793;
    let t35877 = F::cast_from(0.37737710747524982482e-2_f64) * t35876;
    let t35882 = t30598 * t1313 * t721;
    let t35885 = t7859 * t1322 * t721;
    (t35845, t35849, t35851, t35875, t35877, t35882, t35885)
}
