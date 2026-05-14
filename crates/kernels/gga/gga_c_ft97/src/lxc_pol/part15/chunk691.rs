//! GGA_C_FT97 lxc pol — lxc_pol part 15 (v4rho4_4) CSE chunk 691/1067 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part15_v4rho4_4_chunk691<F: Float>(t147: F, t21099: F, t4917: F, t9490: F, t9498: F, t2321: F, t4635: F, t231: F, t5053: F, t1526: F, t17685: F, t17703: F, t2320: F, t342: F, t343: F, t3806: F, t4915: F, t4922: F, t5059: F, t9482: F) -> (F, F, F, F, F, F) {
    let t148 = 10000000.0 <= t147;
    let t21100 = piecewise3(t148, 0.0, t21099);
    let t21103 = t9490 * t4917;
    let t21110 = t9498 * t4917;
    let t21114 = t2321 * t4635;
    let t21118 = t231 * t5053;
    let t21122 = t4915 + t5059 + t9482 - t17685 / 18.0 - t17703 / 6.0 - t1526 * t3806 * t21103 / 9.0 - t1526 * t2320 * t4922 / 6.0 + t1526 * t2320 * t21110 / 6.0 - t1526 * t2320 * t21114 / 12.0 - t342 * t343 * t21118 / 4.0;
    (t21100, t21103, t21110, t21114, t21118, t21122)
}
