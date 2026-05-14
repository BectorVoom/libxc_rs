//! GGA_C_FT97 lxc pol — lxc_pol part 30 (v4rho2sigma2_11) CSE chunk 593/1042 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::expint_e1::{xc_e1_scaled};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_ft97_lxc_pol_part30_v4rho2sigma2_11_chunk593<F: Float>(t28300: F, t3864: F, t28299: F, t6917: F, t9787: F, t1091: F, t24599: F, t2606: F, t24793: F, t3870: F, t6837: F, t729: F, t773: F, t242: F, t27987: F, t1901: F, t24731: F, t24733: F, t24735: F, t28286: F, t28289: F, t28291: F, t28295: F, t446: F) -> (F, F, F) {
    let t28301 = t28300 * t3864;
    let t28302 = t28299 * t28301;
    let t28305 = t9787 * t6917;
    let t28308 = t24599 * t1091;
    let t28309 = t2606 * t28308;
    let t28312 = t24793 * t3870;
    let t28319 = t729 * t773 * t6837;
    let t28322 = t242 * t27987;
    let t28325 = t446 * t28286 / 3.0 + t28289 / 27.0 + t1901 * t28291 / 9.0 - 2.0 / 3.0 * t1901 * t28295 - 2.0 * t1901 * t28302 + t1901 * t28305 / 9.0 + t1901 * t28309 / 9.0 + t1901 * t28312 / 9.0 + t24731 / 9.0 + t24733 / 9.0 + t24735 / 9.0 - t446 * t28319 / 3.0 + 2.0 / 3.0 * t446 * t28322;
    (t28301, t28308, t28325)
}
