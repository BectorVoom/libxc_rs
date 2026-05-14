//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 823/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk823<F: Float>(t18627: F, t242: F, t1882: F, t5070: F, t5181: F, t684: F, t724: F, t10000: F, t14212: F, t14223: F, t14224: F, t14232: F, t14233: F, t18602: F, t18606: F, t18610: F, t18614: F, t18619: F, t18624: F, t446: F) -> (F,) {
    let t18628 = t242 * t18627;
    let t18633 = t1882 * t5070;
    let t18636 = t724 * t5181 * t684;
    let t18639 = 4.0 / 27.0 * t10000 - t446 * t18602 / 9.0 - t446 * t18606 / 9.0 - 2.0 / 27.0 * t446 * t18610 - 2.0 / 9.0 * t446 * t18614 - 2.0 / 3.0 * t446 * t18619 + 4.0 / 3.0 * t446 * t18624 + 2.0 / 3.0 * t446 * t18628 + t14212 - t14223 - 8.0 / 81.0 * t14224 + t14232 - 8.0 / 27.0 * t14233 - 2.0 / 9.0 * t18633 - t446 * t18636 / 9.0;
    (t18639,)
}
