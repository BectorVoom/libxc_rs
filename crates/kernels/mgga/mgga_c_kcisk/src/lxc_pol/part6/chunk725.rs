//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 725/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk725<F: Float>(t12974: F, t311: F, t313: F, t3841: F, t306: F, t315: F, t1170: F, t3675: F, t305: F, t320: F, t3678: F, t330: F, t3721: F) -> (F, F, F, F, F, F, F, F, F) {
    let t12975 = F::new(28.0) / F::new(27.0) * t12974;
    let t12998 = t311 * t3841 * t313;
    let t12999 = F::cast_from(0.36514074074074074075e0_f64) * t12998;
    let t13000 = F::cast_from(0.93011851851851851854e0_f64) * t12974;
    let t13009 = F::new(1.0) / t306 / t315 / F::new(4.0);
    let t13020 = F::new(1.0) / t3675 / t1170;
    let t13021 = t305 * t13020;
    let t13023 = F::new(1.0) / t3678 / t320;
    let t13027 = F::cast_from(0.28842592592592592592e-1_f64) * t12974;
    let t13064 = F::new(1.0) / t3721 / t330;
    (t12975, t12998, t12999, t13000, t13009, t13021, t13023, t13027, t13064)
}
