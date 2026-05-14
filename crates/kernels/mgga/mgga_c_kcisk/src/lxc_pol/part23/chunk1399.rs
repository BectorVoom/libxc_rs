//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1399/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1399<F: Float>(t14294: F, t1520: F, t33636: F, t20922: F, t32241: F, t20919: F, t9509: F, t21341: F, t9483: F, t109171: F, t6244: F, t32308: F, t6241: F, t33618: F, t4321: F, t1458: F, t33616: F) -> (F, F, F, F, F, F, F, F) {
    let t114829 = 12.0 * t14294 * t33636 * t1520;
    let t114831 = 4.0 * t20922 * t32241;
    let t114837 = 2.0 * t20919 * t9509;
    let t114838 = t9483 * t21341;
    let t114840 = 4.0 * t109171 * t6244;
    let t114841 = t6241 * t32308;
    let t114844 = t33618 * t4321;
    let t114849 = t33616 * t1458;
    (t114829, t114831, t114837, t114838, t114840, t114841, t114844, t114849)
}
