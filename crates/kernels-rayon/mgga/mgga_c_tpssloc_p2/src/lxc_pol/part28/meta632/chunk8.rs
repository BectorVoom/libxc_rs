//! MGGA_C_TPSSLOC lxc pol — lxc_pol part 28 (v4rho3sigma_4) CSE chunk 1997/2041 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3, piecewise5};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_tpssloc_lxc_pol_part28_v4rho3sigma_4_chunk1997(t12971: f64, t13471: f64, t13487: f64, t16596: f64, t1877: f64, t193: f64, t202: f64, t2057: f64, t24191: f64, t24339: f64, t24344: f64, t2522: f64, t25365: f64, t26563: f64, t26740: f64, t26744: f64, t4119: f64, t4255: f64, t4303: f64, t4314: f64, t47645: f64, t57912: f64, t57921: f64, t59580: f64, t7110: f64, t7114: f64, t776: f64, t7856: f64, t86706: f64, t870: f64, t89733: f64, t92989: f64) -> f64 {
    let t93052 = -t1877 * t7114 * t13471 + 3.0_f64 * t2522 * t2057 * t12971 + 12.0_f64 * t24191 * t89733 - 12.0_f64 * t26563 * t57912 - 6.0_f64 * t2522 * t26744 * t13487 + 6.0_f64 * t47645 * t7856 + 6.0_f64 * t2522 * t26740 * t776 - 6.0_f64 * t4314 * t7114 * t86706 + 6.0_f64 * t2522 * t7110 * t4119 + t193 * t202 * t92989 * t870 + 6.0_f64 * t2522 * t24344 * t57921 - 6.0_f64 * t2522 * t24339 * t16596 - 6.0_f64 * t2522 * t24339 * t25365 - 3.0_f64 * t2522 * t7114 * t59580 - 2.0_f64 * t1877 * t24339 * t4303 + 12.0_f64 * t4314 * t7110 * t4255;
    t93052
}
