//! GGA_C_GAPLOC lxc pol — lxc_pol part 48 (v4rhosigma3_13) CSE chunk 862/1003 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn gga_c_gaploc_lxc_pol_part48_v4rhosigma3_13_chunk862<F: Float>(t44725: F, t42942: F, t13630: F, t1841: F, t2536: F, t734: F, t42953: F, t2576: F, t35435: F, t161: F, t36610: F, t10678: F, t42931: F, t42933: F, t42936: F, t42939: F, t42951: F, t42956: F, t44711: F, t44716: F, t44719: F, t44723: F, t8878: F) -> F {
    let t44726 = F::cast_from(0.64087718584518535698e-3_f64) * t44725;
    let t44731 = F::cast_from(0.1281754371690370714e-2_f64) * t42942;
    let t44735 = F::cast_from(0.85450291446024714263e-3_f64) * t1841 * t2536 * t13630 * t734;
    let t44740 = F::cast_from(0.17090058289204942853e-2_f64) * t42953;
    let t44744 = F::cast_from(0.59815204012217299984e-2_f64) * t1841 * t35435 * t2576;
    let t44745 = t36610 * t161;
    let t44748 = F::cast_from(0.25635087433807414279e-2_f64) * t1841 * t44745 * t2576;
    let t44749 = t44711 - t44716 + t44719 - t44723 + t44726 - F::cast_from(0.1281754371690370714e-2_f64) * t42931 - F::cast_from(0.3845263115071112142e-2_f64) * t42933 - F::cast_from(0.3845263115071112142e-2_f64) * t42936 - F::cast_from(0.3845263115071112142e-2_f64) * t42939 + t44731 - t44735 + F::cast_from(0.51270174867614828558e-2_f64) * t1841 * t8878 * t10678 - F::cast_from(0.3845263115071112142e-2_f64) * t42951 - t44740 + F::cast_from(0.2563508743380741428e-2_f64) * t42956 + t44744 + t44748;
    t44749
}
