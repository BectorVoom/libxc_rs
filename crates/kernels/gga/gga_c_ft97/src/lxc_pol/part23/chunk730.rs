//! GGA_C_FT97 lxc pol — lxc_pol part 23 (v4rho3sigma_8) CSE chunk 730/1420 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part23_v4rho3sigma_8_chunk730<F: Float>(t18852: F, t898: F, t900: F, t1268: F, t992: F, t505: F, t14514: F, t10864: F, t668: F, t904: F, t14519: F, t4357: F, t4370: F, t2253: F, t5442: F, t10838: F, t10921: F, t14421: F, t14423: F, t14429: F, t14431: F, t14445: F, t14448: F, t14478: F, t14480: F, t14482: F, t18820: F, t18823: F, t18825: F, t2265: F, t631: F) -> (F,) {
    let t18854 = t898 * t900 * t18852;
    let t18857 = t992 * t1268;
    let t18858 = t18857 * t505;
    let t18859 = t14514 * t18858;
    let t18862 = t10864 * t668;
    let t18864 = t18862 * t18857 * t904;
    let t18867 = t14519 * t18858;
    let t18871 = t898 * t4357 * t4370;
    let t18874 = t2253 * t5442;
    let t18876 = t14421 + t14423 + 4.0 / 9.0 * t14429 + 10.0 / 27.0 * t14431 + 10.0 / 9.0 * t14445 - t14448 + t14478 + t14480 - t14482 + t10838 + 5.0 / 27.0 * t10921 - 2.0 / 3.0 * t2265 * t18820 - t18823 / 3.0 + t18825 + t631 * t18854 / 2.0 - 2.0 / 9.0 * t2265 * t18859 + 2.0 * t2265 * t18864 + 4.0 / 3.0 * t2265 * t18867 - 3.0 * t631 * t18871 - t18874 / 27.0;
    (t18876,)
}
