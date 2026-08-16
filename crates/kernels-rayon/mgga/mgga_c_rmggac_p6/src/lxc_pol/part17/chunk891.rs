//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 891/1111 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk891(t2320: f64, t40359: f64, t6355: f64, t8404: f64, t5055: f64, t8407: f64, t2024: f64, t30283: f64, t30360: f64, t30800: f64, t34753: f64, t34757: f64, t34773: f64, t38749: f64, t38757: f64, t38776: f64, t38784: f64, t44925: f64, t44929: f64, t44941: f64, t44949: f64, t4985: f64, t739: f64, t7703: f64, t8387: f64, t884: f64) -> f64 {
    let t44951 = t40359 * t2320;
    let t44954 = t6355 * t8404;
    let t44956 = t5055 * t8407;
    let t44960 = 0.30487649791575028314e-3_f64 * t44925 + 0.30487649791575028314e-3_f64 * t44929 - t34753 - 0.8081505494844540645e-6_f64 * t34757 + 0.30487649791575028314e-3_f64 * t38749 - 0.30487649791575028314e-3_f64 * t38757 - 0.23948483403727617128e0_f64 * t884 * t2024 * t30360 - 0.23948483403727617128e0_f64 * t884 * t2024 * t30283 + t38776 + 0.5987120850931904282e-1_f64 * t44941 - 0.35922725105591425692e0_f64 * t739 * t7703 * t30800 + 0.42564599893297839398e-5_f64 * t44949 - 0.1064114997332445985e-4_f64 * t44951 + 0.20001418546446583934e0_f64 * t38784 - t34773 + 0.5987120850931904282e-1_f64 * t44954 - 0.8980681276397856423e-1_f64 * t44956 - 0.23948483403727617128e0_f64 * t4985 * t8387;
    t44960
}
