//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 723/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk723<F: Float>(t1202: F, t3721: F, t333: F, t3724: F, t317: F, t3675: F, t305: F, t3951: F, t79: F, t222: F, t3531: F) -> (F, F, F, F, F) {
    let t12884 = F::new(1.0) / t3721 / t1202;
    let t12888 = F::new(1.0) / t3724 / t333;
    let t12909 = F::new(1.0) / t3675 / t317;
    let t12910 = t305 * t12909;
    let t12941 = t79 * t3951;
    let t12951 = F::new(1.0) / t3531 / t222;
    (t12884, t12888, t12910, t12941, t12951)
}
