//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 546/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk546<F: Float>(t2379: F, t4939: F, t1096: F, t1113: F, t1614: F, t236: F, t679: F, t3771: F, t6: F, t213: F, t51: F, t1109: F, t1127: F, t680: F, t200: F, t2394: F) -> (F, F, F, F, F, F, F, F, F, F, F) {
    let t4940 = t2379 * t4939;
    let t4943 = t1096 * t1113;
    let t4947 = t236 * t1614;
    let t4948 = t4947 * t679;
    let t4949 = t3771 * t4948;
    let t4950 = t1096 * t6;
    let t4951 = t51 * t213;
    let t4952 = t4951 * t1109;
    let t4953 = t4950 * t4952;
    let t4957 = t680 * t1096 * t1127;
    let t4960 = t4939 * t200;
    let t4961 = t2394 * t4960;
    (t4940, t4943, t4947, t4949, t4950, t4951, t4952, t4953, t4957, t4960, t4961)
}
