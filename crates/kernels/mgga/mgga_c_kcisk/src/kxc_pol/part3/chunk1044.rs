//! MGGA_C_KCISK kxc pol — kxc_pol part 3 (v3rho3_0) CSE chunk 1044/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_kxc_pol_part3_v3rho3_0_chunk1044<F: Float>(t227: F, t15216: F, t15450: F, t218: F, t10449: F, t565: F, t806: F, t564: F, t1629: F, t5556: F, t1009: F, t3179: F, t1053: F, zeta_threshold: F) -> (F, F, F, F) {
    let t228 = t227 <= zeta_threshold;
    let t15451 = t15216 + t15450;
    let t15452 = t15451 * t218;
    let t15453 = piecewise3::<f64>(t228, F::new(0.0), t10449);
    let t15454 = t565 * t15453;
    let t15455 = t15454 * t806;
    let t15456 = t564 * t15455;
    let t15457 = t15456 / F::new(16.0);
    let t15458 = t1629 * t5556;
    let t15459 = t564 * t15458;
    let t15460 = F::new(3.0) / F::new(16.0) * t15459;
    let t15461 = t3179 * t1009;
    let t15462 = t15461 * t1053;
    (t15452, t15457, t15460, t15462)
}
