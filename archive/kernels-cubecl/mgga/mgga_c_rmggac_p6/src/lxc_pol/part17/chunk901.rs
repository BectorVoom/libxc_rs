//! MGGA_C_RMGGAC lxc pol — lxc_pol part 17 (v4rho3sigma_8) CSE chunk 901/1111 (D-02 tuple-return <F: Float>).
#![allow(unused_imports, unused_variables, non_snake_case, clippy::excessive_precision, clippy::too_many_arguments, clippy::needless_return)]

use cubecl::prelude::*;
use libxc_kernel_math::constants::{M_CBRT2, M_CBRT3, M_CBRT4, M_CBRT6, M_PI};
use libxc_kernel_math::piecewise::{piecewise3};
use libxc_kernel_math::powers::{pow_1_3, pow_1_4, pow_3_2};

#[allow(unused_variables, non_snake_case, clippy::too_many_arguments)]
#[cube]
pub fn mgga_c_rmggac_lxc_pol_part17_v4rho3sigma_8_chunk901<F: Float>(t1392: F, t1979: F, t1982: F, t201: F, t597: F, t1451: F, t589: F, t1856: F, t446: F, t38946: F, t38969: F, t38976: F, t38986: F, t38998: F, t39024: F, t39025: F, t39031: F, t42793: F, t45080: F, t45087: F, t45089: F, t45091: F, t45094: F, t6522: F, t739: F, t7567: F) -> F {
    let t45099 = t1392 * t597 * t201 * t1979 * t1982;
    let t45104 = t589 * t1451 * t201 * t1979 * t1982;
    let t45109 = t446 * t1856 * t201 * t1979 * t1982;
    let t45116 = F::cast_from(0.12769379967989351819e-4_f64) * t45080 + F::cast_from(0.72732431077987577944e-1_f64) * t38946 + F::cast_from(0.23948483403727617128e0_f64) * t739 * t7567 * t6522 + F::cast_from(0.10227998120342003148e-1_f64) * t45087 - F::cast_from(0.11918087970123395031e-3_f64) * t45089 - t42793 + t38969 - F::cast_from(0.42564599893297839398e-5_f64) * t45091 - F::cast_from(0.42564599893297839398e-5_f64) * t45094 + F::cast_from(0.85129199786595678796e-5_f64) * t45099 + F::cast_from(0.85129199786595678796e-5_f64) * t45104 + F::cast_from(0.42564599893297839398e-5_f64) * t45109 + F::cast_from(2.0_f64) * t38976 - F::cast_from(0.59590439850616975158e-4_f64) * t38986 - F::cast_from(0.59590439850616975157e-4_f64) * t38998 + t39024 - F::cast_from(0.54549323308490683457e-1_f64) * t39025 - F::cast_from(0.54549323308490683457e-1_f64) * t39031;
    t45116
}
