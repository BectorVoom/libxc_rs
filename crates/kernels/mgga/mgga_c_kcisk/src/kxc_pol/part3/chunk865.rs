//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 865/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk865<F: Float>(t1180: F, t12992: F, t311: F, t313: F, t3841: F, t12974: F, t12983: F, t3661: F, t26: F, t1186: F, t12868: F, t306: F, t315: F) -> (F, F, F, F, F, F, F) {
    let t12995 = t1180 * t12992;
    let t12998 = t311 * t3841 * t313;
    let t12999 = F::new(0.36514074074074074075e0) * t12998;
    let t13000 = F::new(0.93011851851851851854e0) * t12974;
    let t13001 = t3661 * t12983;
    let t13002 = t26 * t13001;
    let t13004 = t1186 * t12868;
    let t13005 = t26 * t13004;
    let t13009 = F::new(1.0) / t306 / t315 / F::new(4.0);
    (t12995, t12998, t12999, t13000, t13002, t13005, t13009)
}
