//! MGGA_C_REVTPSS lxc pol — lxc_pol part 21 (v4rho4_1) CSE chunk 2714/3259 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part21_v4rho4_1_chunk2714<F: Float>(t39774: F, t15071: F, t892: F, t14330: F, t14389: F, t2251: F, t14322: F, t2516: F, t39779: F, t2496: F, t14426: F, t177: F, t762: F) -> (F, F, F, F, F, F, F) {
    let t49945 = F::cast_from(0.17544670867903938621e1_f64) * t39774;
    let t49950 = t15071 * t892;
    let t49956 = F::new(72.0) * t14330 * t14389 * t2251;
    let t49957 = t14322 * t2516;
    let t49958 = F::cast_from(0.17544670867903938621e1_f64) * t49957;
    let t49959 = F::new(3.0) * t39779;
    let t49963 = t14322 * t2496;
    let t49964 = F::cast_from(0.51947577317044391276e2_f64) * t49963;
    let t49966 = t14426 * t177 * t762;
    (t49945, t49950, t49956, t49958, t49959, t49964, t49966)
}
