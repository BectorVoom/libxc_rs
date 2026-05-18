//! MGGA_C_KCIS lxc pol — lxc_pol part 5 (v3rho3_2) CSE chunk 1022/1419 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_lxc_pol_part5_v3rho3_2_chunk1022<F: Float>(t14447: F, t4567: F, t991: F, t2469: F, t992: F, t4952: F, t291: F, t9897: F, t2887: F, t736: F, t1245: F, t4967: F) -> (F, F, F, F, F, F) {
    let t14448 = t14447 * t4567;
    let t14450 = t991 * t14448 / F::new(324.0);
    let t14453 = t2469 * t992;
    let t14454 = t14453 * t4952;
    let t14455 = t991 * t14454;
    let t14492 = t9897 * t291;
    let t14496 = t736 * t2887;
    let t14497 = t14496 * t291;
    let t14516 = t1245 * t4967;
    (t14450, t14455, t14492, t14496, t14497, t14516)
}
