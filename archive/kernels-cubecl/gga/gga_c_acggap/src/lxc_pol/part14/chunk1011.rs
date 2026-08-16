//! GGA_C_ACGGAP lxc pol — lxc_pol part 14 (v4rho3sigma_6) CSE chunk 1011/1223 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_acggap_lxc_pol_part14_v4rho3sigma_6_chunk1011<F: Float>(t35682: F, t31773: F, t8634: F, t2288: F, t4210: F, t15386: F, t31057: F, t1347: F, t7614: F, t1967: F, t8502: F, t1998: F, t5089: F) -> (F, F, F, F, F, F, F) {
    let t35683 = F::cast_from(0.28582678745379824648e-3_f64) * t35682;
    let t35685 = t31773 * t8634;
    let t35686 = F::cast_from(11.0_f64) / F::cast_from(48.0_f64) * t35685;
    let t35700 = t2288 * t4210;
    let t35702 = t31057 * t15386 * t35700;
    let t35703 = F::cast_from(0.94344276868812456204e-3_f64) * t35702;
    let t35709 = t7614 * t1347;
    let t35710 = F::cast_from(0.32012600194825403606e-1_f64) * t35709;
    let t35722 = t1967 * t8502;
    let t35723 = F::cast_from(0.25724410870841842184e-2_f64) * t35722;
    let t35733 = t1998 * t5089;
    (t35683, t35686, t35700, t35703, t35710, t35723, t35733)
}
