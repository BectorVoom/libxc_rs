//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1061/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1061<F: Float>(t15781: F, t15792: F, t44: F, t291: F, t15174: F, t15452: F, t15457: F, t15460: F, t15463: F, t15466: F, t15471: F, t15473: F, t15763: F, t15766: F, t15767: F, t15770: F) -> F {
    let t15794 = (t15781 + t15792) * t44;
    let t15795 = t15794 * t291;
    let t15796 = -t15174 + t15452 - t15457 - t15460 - t15463 - t15466 - t15471 - t15473 + t15763 - t15766 + F::new(3.0) * t15767 - t15770 + t15795;
    t15796
}
