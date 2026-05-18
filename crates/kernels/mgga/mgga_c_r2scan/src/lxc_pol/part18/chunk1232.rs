//! MGGA_C_R2SCAN lxc pol — lxc_pol part 18 (v4rho3sigma_8) CSE chunk 1232/1264 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_r2scan_lxc_pol_part18_v4rho3sigma_8_chunk1232<F: Float>(t12520: F, t1584: F, t12463: F, t2207: F, t3336: F, t38183: F, t38190: F, t40258: F, t40261: F, t43688: F, t43690: F, t43692: F, t43695: F, t43697: F, t43700: F) -> F {
    let t43702 = t1584 * t12520;
    let t43705 = t2207 * t3336 * t12463;
    let t43707 = -F::new(0.54878743191129263322e-1) * t43688 + F::new(0.86682217400542685632e-1) * t43690 + F::new(0.29272321618148349057e-1) * t43692 - F::new(0.16463622957338778997e-1) * t38183 + t38190 + t40258 - F::new(0.12805040077930161442e0) * t43695 - F::new(0.43341108700271342816e-1) * t43697 - F::new(0.43341108700271342816e-1) * t43700 - F::new(0.43341108700271342816e-1) * t43702 - t40261 + F::new(0.65495539973149862688e-2) * t43705;
    t43707
}
