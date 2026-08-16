//! MGGA_C_KCIS kxc pol — kxc_pol part 4 (v3rho3_1) CSE chunk 1192/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcis_kxc_pol_part4_v3rho3_1_chunk1192<F: Float>(t15220: F, t4567: F, t3514: F, t13462: F, t5302: F, t421: F, t9897: F, t13467: F, t14496: F, t13516: F, t1662: F, t3611: F) -> (F, F, F, F, F) {
    let t15221 = t15220 * t4567;
    let t15223 = t3514 * t15221 / F::cast_from(648.0_f64);
    let t15224 = t5302 * t13462;
    let t15227 = t9897 * t421;
    let t15228 = t15227 * t13467;
    let t15231 = t14496 * t421;
    let t15232 = t15231 * t13516;
    let t15235 = t1662 * t3611;
    (t15223, t15224, t15228, t15232, t15235)
}
