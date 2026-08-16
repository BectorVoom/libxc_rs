//! GGA_C_GAPC lxc pol — lxc_pol part 28 (v4rho2sigma2_7) CSE chunk 1189/1429 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part28_v4rho2sigma2_7_chunk1189<F: Float>(t11784: F, t11983: F, t3784: F, t3788: F, t7241: F, t11990: F, t19196: F, t2597: F, t1086: F, t11790: F, t22581: F, t17760: F, t2580: F, t33273: F) -> (F, F, F, F, F) {
    let t33943 = t11784 * t11983;
    let t33946 = t3784 * t7241 * t3788;
    let t33949 = t11990 * t2597 * t19196;
    let t33952 = t11790 * t1086 * t22581;
    let t33956 = t17760 * t33273 * t2580;
    (t33943, t33946, t33949, t33952, t33956)
}
