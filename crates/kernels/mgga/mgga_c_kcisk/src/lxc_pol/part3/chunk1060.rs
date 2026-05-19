//! MGGA_C_KCISK lxc pol — lxc_pol part 3 (v3rho3_0) CSE chunk 1060/1063 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part3_v3rho3_0_chunk1060<F: Float>(t222: F, t227: F, t12830: F, t12924: F, t15772: F, t15775: F, t224: F, t3283: F, t3288: F, t1060: F, t3289: F, t10441: F, t10449: F, t229: F, t3293: F, zeta_threshold: F) -> (F, F) {
    let t223 = t222 <= zeta_threshold;
    let t228 = t227 <= zeta_threshold;
    let t15781 = piecewise3::<F>(t223, F::new(0.0), -F::new(8.0) / F::new(27.0) * t15772 * t12830 + F::new(4.0) / F::new(3.0) * t15775 * t3283 + F::new(4.0) / F::new(3.0) * t224 * t12924);
    let t15783 = F::new(1.0) / t3288 / t227;
    let t15786 = t3289 * t1060;
    let t15792 = piecewise3::<F>(t228, F::new(0.0), -F::new(8.0) / F::new(27.0) * t15783 * t10441 + F::new(4.0) / F::new(3.0) * t15786 * t3293 + F::new(4.0) / F::new(3.0) * t229 * t10449);
    (t15781, t15792)
}
