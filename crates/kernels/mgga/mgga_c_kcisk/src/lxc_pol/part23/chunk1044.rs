//! MGGA_C_KCISK lxc pol — lxc_pol part 23 (v4rho3sigma_3) CSE chunk 1044/1447 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part23_v4rho3sigma_3_chunk1044<F: Float>(t14592: F, t6328: F, t1513: F, t6344: F, t1501: F, t6363: F, t1497: F, t6310: F, t1286: F, t5991: F, t4204: F, t6331: F, t19904: F, t6317: F, t6316: F, t19881: F, t6443: F) -> (F, F, F, F, F, F, F, F, F) {
    let t20949 = t14592 * t6328;
    let t20951 = t6344 * t1513;
    let t20953 = t1501 * t6363;
    let t20955 = t6310 * t1497;
    let t20957 = t5991 * t1286;
    let t20958 = t4204 * t20957;
    let t20959 = t6331 * t20958;
    let t20961 = t6317 * t19904;
    let t20962 = t6316 * t20961;
    let t20964 = t6443 * t19881;
    (t20949, t20951, t20953, t20955, t20957, t20959, t20961, t20962, t20964)
}
