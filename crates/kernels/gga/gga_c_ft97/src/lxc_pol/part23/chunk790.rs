//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 790/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk790<F: Float>(t19372: F, t19428: F, t19478: F, t19531: F, t19584: F, t19791: F, t19809: F, t19880: F, t19782: F, t312: F, t5478: F, t909: F, t4381: F, t505: F, t5474: F, t4917: F, t9490: F) -> (F, F, F, F, F) {
    let t19883 = t19372 + t19428 + t19478 + t19531 + t19584 + t19791 + t19809 + t19880;
    let t19886 = t19782 * t312;
    let t19905 = t5478 * t909;
    let t19906 = t19905 * t4381;
    let t19927 = t5474 * t505;
    let t21103 = t9490 * t4917;
    (t19883, t19886, t19906, t19927, t21103)
}
