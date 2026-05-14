//! MGGA_C_KCISK lxc pol — lxc_pol part 26 (v4rho3sigma_6) CSE chunk 873/1407 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part26_v4rho3sigma_6_chunk873<F: Float>(t1390: F, t1433: F, t2079: F, t3676: F, t5741: F, t827: F, t5738: F, t19104: F, t2089: F, t2877: F, t16391: F, t5745: F, t19102: F, t3638: F, t1171: F, t5712: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t19450 = t1433 * t1390;
    let t19476 = t2079 * t3676;
    let t19483 = t827 * t5741;
    let t19484 = 0.21908444444444444444e0 * t19483;
    let t19485 = t827 * t5738;
    let t19540 = 0.39862222222222222222e0 * t19104;
    let t19543 = t2877 * t2089;
    let t19545 = t16391 * t5745;
    let t19565 = 0.41203703703703703704e-2 * t19102;
    let t19566 = 0.12361111111111111111e-1 * t19104;
    let t19580 = t2079 * t3638;
    let t19583 = t5712 * t1171;
    (t19450, t19476, t19483, t19484, t19485, t19540, t19543, t19545, t19565, t19566, t19580, t19583)
}
