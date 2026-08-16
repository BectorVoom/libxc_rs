//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 21 (v4rho4_2) CSE chunk 2471/3221 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_tpssloc_lxc_pol_part21_v4rho4_2_chunk2471<F: Float>(t3030: F, t3481: F, t3032: F, t3505: F, t3514: F, t11147: F, t3439: F, t11789: F, t820: F, t3577: F, t3579: F, t11737: F, t44857: F) -> (F, F, F, F, F, F, F) {
    let t44927 = t3481 * t3030;
    let t44928 = t44927 * t3032;
    let t44929 = t44928 * t3505;
    let t44932 = t44928 * t3514;
    let t44938 = t3439 * t11147;
    let t44951 = t820 * t11789;
    let t44953 = t3577 * t44951 * t3579;
    let t44965 = t44857 * t11737;
    (t44927, t44929, t44932, t44938, t44951, t44953, t44965)
}
