//! GGA_C_FT97 kxc pol — kxc_pol part 3 (v3rho3_2) CSE chunk 595/1032 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_kxc_pol_part3_v3rho3_2_chunk595<F: Float>(t4950: F, t4952: F, t1096: F, t1127: F, t680: F, t200: F, t4939: F, t2394: F, t2347: F, t4917: F) -> (F, F, F, F, F) {
    let t4953 = t4950 * t4952;
    let t4957 = t680 * t1096 * t1127;
    let t4960 = t4939 * t200;
    let t4961 = t2394 * t4960;
    let t4965 = t2347 * t4917;
    (t4953, t4957, t4960, t4961, t4965)
}
