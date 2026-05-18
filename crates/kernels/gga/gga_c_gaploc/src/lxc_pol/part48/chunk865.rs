//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 865/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk865<F: Float>(t2932: F, t44787: F, t9647: F, t11680: F, t40820: F, t7064: F, t123: F, t1841: F, t1843: F, t42960: F, t42967: F, t42970: F, t42985: F, t42988: F, t42991: F, t44751: F, t44756: F, t44759: F, t44762: F, t44764: F, t44766: F, t44772: F, t44776: F, t44780: F, t44786: F, t734: F) -> F {
    let t44789 = t9647 * t2932 * t44787;
    let t44790 = F::new(0.64087718584518535698e-3) * t44789;
    let t44792 = t7064 * t11680 * t40820;
    let t44794 = -F::new(0.8972280601832594998e-2) * t42960 + t44751 - F::new(0.7690526230142224284e-2) * t42967 - F::new(0.2563508743380741428e-2) * t42970 + t44756 - t44759 - t44762 + F::new(0.12817543716903707139e-2) * t44764 - F::new(0.85450291446024714263e-3) * t1841 * t44766 * t123 * t734 + F::new(0.85450291446024714263e-3) * t1841 * t1843 * t44772 - t44776 - t44780 + F::new(0.2563508743380741428e-2) * t42985 + F::new(0.2563508743380741428e-2) * t42988 + F::new(0.2563508743380741428e-2) * t42991 - t44786 + t44790 + F::new(0.96131577876777803546e-3) * t44792;
    t44794
}
