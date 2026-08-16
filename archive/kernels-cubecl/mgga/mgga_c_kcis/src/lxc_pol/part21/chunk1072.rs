//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1072/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1072<F: Float>(t1009: F, t2844: F, t2630: F, t4939: F, t10454: F, t922: F, t4947: F, t2635: F, t3203: F, t7718: F, t1020: F, t4555: F) -> (F, F, F, F, F, F, F, F, F) {
    let t26695 = t1009 * t2844;
    let t26696 = t26695 * t2630;
    let t26697 = t4939 * t26696;
    let t26702 = t10454 * t922;
    let t26703 = t4947 * t26702;
    let t26706 = t3203 * t2635;
    let t26707 = t7718 * t26706;
    let t26708 = t1020 * t26707;
    let t26710 = t4555 * t2630;
    (t26695, t26696, t26697, t26702, t26703, t26706, t26707, t26708, t26710)
}
