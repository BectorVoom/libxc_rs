//! GGA_C_GAPC lxc pol — lxc_pol part 37 (v4rho2sigma2_16) CSE chunk 887/1445 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gapc_lxc_pol_part37_v4rho2sigma2_16_chunk887<F: Float>(t10072: F, t3411: F, t1084: F, t8711: F, t134: F, t7877: F, t442: F, t7591: F, t941: F, t10024: F, t10027: F, t10032: F, t10037: F, t10041: F, t10044: F, t10048: F, t10051: F, t10054: F, t10059: F, t10064: F, t10070: F) -> (F, F, F, F, F) {
    let t10073 = t3411 * t10072;
    let t10075 = t1084 * t8711;
    let t10077 = t134 * t7877;
    let t10078 = t10077 * t442;
    let t10079 = t7591 * t941 * t10078;
    let t10080 = t10075 * t10079;
    let t10082 = -F::new(0.2204045389310251527e-6) * t10024 + F::new(0.6487109086417285278e-2) * t10027 + F::new(0.50027140879067581468e-8) * t10032 - F::new(0.16882049790461501058e-6) * t10037 - F::new(0.56273499301538336859e-8) * t10041 + F::new(0.4103275990737170396e-9) * t10044 - F::new(0.2813674965076916843e-7) * t10048 + F::new(0.10120442708333333334e-4) * t10051 - F::new(0.24619655944423022376e-7) * t10054 - F::new(0.24619655944423022376e-7) * t10059 + F::new(0.84410248952307505288e-7) * t10064 - F::new(0.25323074685692251586e-6) * t10070 + F::new(0.16882049790461501058e-6) * t10073 - F::new(0.17951832459475120482e-8) * t10080;
    (t10073, t10078, t10079, t10080, t10082)
}
