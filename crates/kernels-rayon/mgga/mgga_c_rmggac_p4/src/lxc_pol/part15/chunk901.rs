//! MGGA_C_RMGGAC lxc pol — lxc_pol part 15 (v4rho3sigma_6) CSE chunk 901/1110 (D-02 tuple-return ).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use libxc_rkernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_rkernel_math::piecewise::{piecewise3};
use libxc_rkernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
pub fn mgga_c_rmggac_lxc_pol_part15_v4rho3sigma_6_chunk901(t1392: f64, t1979: f64, t1982: f64, t201: f64, t597: f64, t1451: f64, t589: f64, t1856: f64, t446: f64, t38946: f64, t38969: f64, t38976: f64, t38986: f64, t38998: f64, t39024: f64, t39025: f64, t39031: f64, t42793: f64, t45080: f64, t45087: f64, t45089: f64, t45091: f64, t45094: f64, t6522: f64, t739: f64, t7567: f64) -> f64 {
    let t45099 = t1392 * t597 * t201 * t1979 * t1982;
    let t45104 = t589 * t1451 * t201 * t1979 * t1982;
    let t45109 = t446 * t1856 * t201 * t1979 * t1982;
    let t45116 = 0.12769379967989351819e-4_f64 * t45080 + 0.72732431077987577944e-1_f64 * t38946 + 0.23948483403727617128e0_f64 * t739 * t7567 * t6522 + 0.10227998120342003148e-1_f64 * t45087 - 0.11918087970123395031e-3_f64 * t45089 - t42793 + t38969 - 0.42564599893297839398e-5_f64 * t45091 - 0.42564599893297839398e-5_f64 * t45094 + 0.85129199786595678796e-5_f64 * t45099 + 0.85129199786595678796e-5_f64 * t45104 + 0.42564599893297839398e-5_f64 * t45109 + 2.0_f64 * t38976 - 0.59590439850616975158e-4_f64 * t38986 - 0.59590439850616975157e-4_f64 * t38998 + t39024 - 0.54549323308490683457e-1_f64 * t39025 - 0.54549323308490683457e-1_f64 * t39031;
    t45116
}
