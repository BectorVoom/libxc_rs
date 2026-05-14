//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1145/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1145<F: Float>(t32057: F, t32117: F, t32167: F, t32222: F, t504: F, t1458: F, t9481: F, t1520: F, t2726: F, t4169: F, t4171: F, t4321: F, t9483: F, t14284: F, t2732: F, t14287: F, t9486: F) -> (F, F, F, F, F, F, F, F, F) {
    let t32224 = t32057 + t32117 + t32167 + t32222;
    let t32225 = t32224 * t504;
    let t32226 = t9481 * t1458;
    let t32228 = 2.0 * t32226 * t1520;
    let t32229 = t2726 * t4169;
    let t32231 = 2.0 * t32229 * t4171;
    let t32232 = t9483 * t4321;
    let t32233 = t14284 * t2732;
    let t32235 = 4.0 * t14287 * t9486;
    (t32224, t32225, t32226, t32228, t32229, t32231, t32232, t32233, t32235)
}
