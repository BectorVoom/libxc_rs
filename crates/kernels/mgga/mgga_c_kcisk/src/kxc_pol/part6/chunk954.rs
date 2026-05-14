//! MGGA_C_KCISK kxc pol — kxc_pol part 6 (v3rho3_3) CSE chunk 954/957 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part6_v3rho3_3_chunk954<F: Float>(t31776: F, t31795: F, t15094: F, t1611: F, t21345: F, t2347: F, t28036: F, t31197: F, t31199: F, t31201: F, t31203: F, t31206: F, t31209: F, t31406: F, t31744: F, t31752: F, t31755: F, t4535: F, t555: F, t6604: F, t8436: F, t8455: F) -> (F,) {
    let t31796 = t31776 + t31795;
    let t31798 = -6.0 * t15094 * t31752 - t1611 * t31796 + 6.0 * t21345 * t8436 - 3.0 * t2347 * t28036 + t31744 * t555 + 6.0 * t31755 * t4535 - 3.0 * t6604 * t8455 - t31197 + t31199 - t31201 + t31203 + t31206 - t31209 + t31406;
    (t31798,)
}
