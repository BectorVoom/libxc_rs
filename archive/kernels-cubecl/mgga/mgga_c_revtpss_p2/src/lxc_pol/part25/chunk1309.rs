//! MGGA_C_REVTPSS lxc pol — lxc_pol part 25 (v4rho3sigma_0) CSE chunk 1309/1360 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_PI};
use libxc_kernel_math::piecewise::{piecewise3, piecewise5};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_revtpss_lxc_pol_part25_v4rho3sigma_0_chunk1309<F: Float>(t2482: F, t596: F, t7269: F, t3981: F, t7271: F, t9944: F, t25986: F, t2661: F, t9930: F, t9757: F, t94418: F, t94420: F, t94424: F, t94426: F, t94430: F, t94432: F, t94434: F, t94436: F, t94438: F, t94440: F) -> F {
    let t94443 = t2482 * t7269 * t596;
    let t94444 = t94443 * t3981;
    let t94446 = t7271 * t9944;
    let t94449 = t2661 * t25986 * t9930;
    let t94451 = t7271 * t9757;
    let t94453 = F::cast_from(0.51448821741683684367e-2_f64) * t94418 + F::cast_from(0.51448821741683684367e-2_f64) * t94420 + F::cast_from(0.6098400337114239387e-3_f64) * t94424 - F::cast_from(0.25724410870841842184e-1_f64) * t94426 - F::cast_from(0.48018900292238105409e-1_f64) * t94430 - F::cast_from(0.10289764348336736873e-1_f64) * t94432 + F::cast_from(0.25724410870841842183e-2_f64) * t94434 - F::cast_from(0.12862205435420921092e-2_f64) * t94436 + F::cast_from(0.51448821741683684367e-2_f64) * t94438 + F::cast_from(0.25724410870841842183e-2_f64) * t94440 + F::cast_from(0.32524801797942610064e-2_f64) * t94444 - F::cast_from(0.51448821741683684367e-1_f64) * t94446 + F::cast_from(0.42874018118069736972e-4_f64) * t94449 - F::cast_from(0.17149607247227894789e-2_f64) * t94451;
    t94453
}
