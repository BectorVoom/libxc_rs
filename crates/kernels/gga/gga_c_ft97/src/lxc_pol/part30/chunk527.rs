//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 527/1184 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk527<F: Float>(t39: F, t695: F, t224: F, t3781: F, t7853: F, t1526: F, t5198: F, t9483: F, t10915: F, t294: F, t3691: F, t2917: F) -> (F, F, F, F, F, F) {
    let t17986 = t695 * t39;
    let t17987 = t224 * t17986;
    let t17994 = t7853 * t3781;
    let t18959 = t1526 * t9483 * t5198;
    let t18961 = t10915 * t294;
    let t18962 = t18961 * t3691;
    let t18968 = t2917 * t294;
    (t17986, t17987, t17994, t18959, t18962, t18968)
}
