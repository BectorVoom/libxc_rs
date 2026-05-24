//! MGGA_C_KCISK lxc pol — lxc_pol part 6 (v3rho3_3) CSE chunk 595/1086 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_kcisk_lxc_pol_part6_v3rho3_3_chunk595<F: Float>(t1312: F, t8331: F, t2326: F, t4375: F, t1586: F, t4423: F, t5668: F, t7738: F, t7742: F, t7746: F, t2292: F, t1537: F) -> (F, F, F, F, F, F, F) {
    let t8332 = t1312 * t8331;
    let t8335 = t2326 * t2326;
    let t8336 = t4375 * t8335;
    let t8337 = t1586 * t8336;
    let t8344 = t4423 + F::cast_from(0.11415555555555555555e-1_f64) * t5668 - F::cast_from(0.11415555555555555555e-1_f64) * t7738 + F::cast_from(0.34246666666666666666e-1_f64) * t7742 - F::cast_from(0.17123333333333333333e-1_f64) * t7746;
    let t8349 = t2292 * t2292;
    let t8350 = t8349 * t1537;
    (t8332, t8335, t8336, t8337, t8344, t8349, t8350)
}
