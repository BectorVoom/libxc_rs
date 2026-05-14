//! GGA_C_FT97 lxc pol — lxc_pol part 3 (v3rho3_2) CSE chunk 842/887 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part3_v3rho3_2_chunk842<F: Float>(t19007: F, t446: F, t5299: F, t668: F, t505: F, t2665: F, t5225: F, t10248: F, t4969: F, t824: F, t17744: F, t835: F, t17780: F, t3281: F, t4973: F, t10279: F, t10400: F, t14636: F, t14638: F, t14640: F, t14658: F, t14684: F, t14718: F, t14903: F, t15111: F, t15116: F, t18999: F, t19004: F) -> (F, F, F, F, F, F, F, F, F, F, F, F) {
    let t19008 = t446 * t19007;
    let t19010 = t5299 * t668;
    let t19011 = t19010 * t505;
    let t19012 = t2665 * t19011;
    let t19013 = t446 * t19012;
    let t19015 = t5225 * t668;
    let t19016 = t19015 * t505;
    let t19017 = t10248 * t19016;
    let t19018 = t446 * t19017;
    let t19020 = t4969 * t824;
    let t19021 = t2665 * t19020;
    let t19022 = t446 * t19021;
    let t19024 = t835 * t17744;
    let t19025 = t446 * t19024;
    let t19027 = t835 * t17780;
    let t19028 = t3281 * t19027;
    let t19030 = t4973 * t824;
    let t19031 = t2665 * t19030;
    let t19032 = t446 * t19031;
    let t19034 = -t14636 - t14638 + t14640 - t14658 - t14684 - 2.0 / 27.0 * t10400 - 2.0 / 81.0 * t10279 - t15111 - 2.0 / 27.0 * t14718 - 2.0 / 9.0 * t18999 - 2.0 / 9.0 * t19004 + 2.0 / 27.0 * t19008 - t15116 + t14903 + t19013 / 18.0 - t19018 / 9.0 - t19022 / 9.0 - t19025 / 3.0 - 4.0 / 9.0 * t19028 + t19032 / 18.0;
    (t19008, t19011, t19013, t19016, t19018, t19020, t19022, t19025, t19028, t19030, t19032, t19034)
}
