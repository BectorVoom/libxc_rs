//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 865/1003 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk865(t2932: f64, t44787: f64, t9647: f64, t11680: f64, t40820: f64, t7064: f64, t123: f64, t1841: f64, t1843: f64, t42960: f64, t42967: f64, t42970: f64, t42985: f64, t42988: f64, t42991: f64, t44751: f64, t44756: f64, t44759: f64, t44762: f64, t44764: f64, t44766: f64, t44772: f64, t44776: f64, t44780: f64, t44786: f64, t734: f64) -> f64 {
    let t44789 = t9647 * t2932 * t44787;
    let t44790 = 0.64087718584518535698e-3_f64 * t44789;
    let t44792 = t7064 * t11680 * t40820;
    let t44794 = -0.8972280601832594998e-2_f64 * t42960 + t44751 - 0.7690526230142224284e-2_f64 * t42967 - 0.2563508743380741428e-2_f64 * t42970 + t44756 - t44759 - t44762 + 0.12817543716903707139e-2_f64 * t44764 - 0.85450291446024714263e-3_f64 * t1841 * t44766 * t123 * t734 + 0.85450291446024714263e-3_f64 * t1841 * t1843 * t44772 - t44776 - t44780 + 0.2563508743380741428e-2_f64 * t42985 + 0.2563508743380741428e-2_f64 * t42988 + 0.2563508743380741428e-2_f64 * t42991 - t44786 + t44790 + 0.96131577876777803546e-3_f64 * t44792;
    t44794
}
