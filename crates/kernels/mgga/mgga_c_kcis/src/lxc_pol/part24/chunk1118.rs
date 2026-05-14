//! MGGA_C_KCIS lxc pol — lxc_pol part 24 (v4rho3sigma_6) CSE chunk 1118/1171 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part24_v4rho3sigma_6_chunk1118<F: Float>(t2822: F, t28924: F, t1020: F, t19135: F, t26760: F, t19140: F, t4994: F, t19785: F, t1092: F, t19715: F, t27763: F, t19576: F, t19815: F, t42530: F, t7718: F, t27836: F, t4796: F) -> (F, F, F, F, F, F, F, F) {
    let t100514 = t2822 * t28924;
    let t100519 = t1020 * t26760 * t19135;
    let t100522 = t4994 * t26760 * t19140;
    let t100525 = t1020 * t26760 * t19785;
    let t100528 = t1092 * t27763 * t19715;
    let t100531 = t1092 * t26760 * t19576;
    let t100540 = t42530 * t7718 * t19815;
    let t100547 = t1020 * t27836 * t4796;
    (t100514, t100519, t100522, t100525, t100528, t100531, t100540, t100547)
}
