//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 955/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk955(t2412: f64, t8582: f64, t2191: f64, t9790: f64, t2024: f64, t30344: f64, t30400: f64, t35478: f64, t35481: f64, t35484: f64, t35487: f64, t35497: f64, t39841: f64, t39842: f64, t39874: f64, t45825: f64, t45827: f64, t45830: f64, t45832: f64, t45836: f64, t5016: f64, t739: f64, t9840: f64) -> f64 {
    let t45844 = t2412 * t8582;
    let t45846 = t2191 * t9790;
    let t45854 = 0.85129199786595678796e-5_f64 * t45825 + 0.1064114997332445985e-4_f64 * t45827 - t39841 + 0.59590439850616975157e-4_f64 * t39842 + 0.6818665413561335432e-1_f64 * t45830 + 0.68186654135613354322e-2_f64 * t45832 - 0.51077519871957407276e-4_f64 * t45836 + 0.23948483403727617128e0_f64 * t739 * t2024 * t30344 + 0.23948483403727617128e0_f64 * t739 * t2024 * t30400 + t39874 - 0.85129199786595678796e-5_f64 * t45844 - 0.42564599893297839398e-5_f64 * t45846 - 0.11974241701863808564e0_f64 * t5016 * t9840 + 0.81300399444200075504e-3_f64 * t35478 - 0.1951603679568577289e-3_f64 * t35481 + 0.81300399444200075504e-3_f64 * t35484 - 0.1951603679568577289e-3_f64 * t35487 + t35497;
    t45854
}
