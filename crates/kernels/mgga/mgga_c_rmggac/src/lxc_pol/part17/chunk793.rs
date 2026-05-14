//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 793/964 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk793<F: Float>(t6355: F, t8404: F, t5055: F, t8407: F, t2024: F, t30283: F, t30360: F, t30800: F, t34753: F, t34757: F, t34773: F, t38749: F, t38757: F, t38776: F, t38784: F, t44925: F, t44929: F, t44941: F, t44949: F, t44951: F, t4985: F, t739: F, t7703: F, t8387: F, t884: F) -> (F,) {
    let t44954 = t6355 * t8404;
    let t44956 = t5055 * t8407;
    let t44960 = 0.30487649791575028314e-3 * t44925 + 0.30487649791575028314e-3 * t44929 - t34753 - 0.8081505494844540645e-6 * t34757 + 0.30487649791575028314e-3 * t38749 - 0.30487649791575028314e-3 * t38757 - 0.23948483403727617128e0 * t884 * t2024 * t30360 - 0.23948483403727617128e0 * t884 * t2024 * t30283 + t38776 + 0.5987120850931904282e-1 * t44941 - 0.35922725105591425692e0 * t739 * t7703 * t30800 + 0.42564599893297839398e-5 * t44949 - 0.1064114997332445985e-4 * t44951 + 0.20001418546446583934e0 * t38784 - t34773 + 0.5987120850931904282e-1 * t44954 - 0.8980681276397856423e-1 * t44956 - 0.23948483403727617128e0 * t4985 * t8387;
    (t44960,)
}
