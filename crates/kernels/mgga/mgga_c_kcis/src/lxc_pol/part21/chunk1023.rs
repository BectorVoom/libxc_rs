//! MGGA_C_KCIS lxc pol — lxc_pol part 21 (v4rho3sigma_3) CSE chunk 1023/1389 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part21_v4rho3sigma_3_chunk1023<F: Float>(t1851: F, t2844: F, t2630: F, t11020: F, t421: F, t4951: F, t13511: F, t1662: F, t3532: F, t11072: F, t3490: F, t5299: F) -> (F, F, F, F, F) {
    let t15529 = t1851 * t2844;
    let t15530 = t15529 * t2630;
    let t15531 = t11020 * t15530;
    let t15534 = t4951 * t421;
    let t15535 = t15534 * t13511;
    let t15540 = t1662 * t3532;
    let t15541 = t11072 * t15540;
    let t15547 = t3490 * t5299 / F::new(324.0);
    (t15531, t15534, t15535, t15541, t15547)
}
